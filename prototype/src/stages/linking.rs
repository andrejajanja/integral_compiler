#![allow(unused_assignments)]

use std::fmt::Error;

use crate::parts::object_type_definitions::*;
use libc::c_void;

type CompiledFunc = extern "C" fn(f64) -> f64;

pub fn sin(n: f64) -> f64{
    f64::sin(n)
}

pub fn cos(n: f64) -> f64{
    f64::cos(n)
}

pub fn exp(n: f64) -> f64{
    f64::exp(n)
}

pub fn tan(n: f64) -> f64{
    f64::tan(n)
}

type FunctionPtr = fn(f64) -> f64;

fn parse_symbol_table<'a>(symbols: &mut Vec<&'a str>, sym_table: Option<&[u8]>, object_file_buffer: &'a [u8], string_table_start: usize) -> usize{
    let mut temp_fja_offset: u64 = u64::MAX;
    match sym_table {
        Some(sym_t) => {
            let mut entry_offset = 0 as usize;
            while entry_offset < sym_t.len(){
                let mut symbol_name: &'a str = "";
            
                let offset_of_string_name = u32::from_le_bytes(sym_t[entry_offset..entry_offset + 4].try_into().expect("Slice with incorrect length")) as usize;
                let symbol_name_buffer = &object_file_buffer[string_table_start + offset_of_string_name..];
                if let Some(null_terminator_index) = symbol_name_buffer.iter().position(|&b| b == 0) {
                    symbol_name = std::str::from_utf8(&symbol_name_buffer[..null_terminator_index])
                        .expect("Failed to convert bytes to string");
                }

                if symbol_name == "fja" {
                    temp_fja_offset = u64::from_le_bytes(sym_t[(entry_offset+8)..(entry_offset+16)].try_into().expect("Slice with incorrect length"));
                }

                symbols.push(symbol_name);
                entry_offset+=24;
            }

            if temp_fja_offset == u64::MAX {panic!("Fja symbol wasn't found in the provided symbol table.")}
            return temp_fja_offset as usize;
        }
        None => {
            panic!("Symbol table wasn't found in the byte buffer provided as 'buffer: &[u8]' arguent");
        }
    }
}

fn resolve_relative_offset(symbol_offset: usize, symbol_name: &str, fc_offset: usize, buffer_pointer: *mut u8) -> i32{
    if symbol_name.contains(".LCPI") {
        if let Some(pos) = symbol_name.find('_') {
            match &symbol_name[pos + 1..].parse::<i32>() {
                Ok(value) => {
                    return ((fc_offset as i32) + value*8) - (symbol_offset as i32);
                },
                Err(e) => println!("Failed to parse usize: {}", e),
            }
        }
    }
    unsafe {
        let pointer_addr = buffer_pointer.add(symbol_offset) as i32;
        match symbol_name{
            "sin" => {return sin as i32 - pointer_addr}
            "cos" => {return cos as i32 - pointer_addr}
            "tan" => {return tan as i32 - pointer_addr}
            "exp" => {return exp as i32 - pointer_addr}
            _ => {panic!("Unrecognized provided symbol: {}", symbol_name);}
        }
    }
}

pub fn link_buffer(buffer: &mut[u8], buffer_pointer: *mut u8) -> FunctionPtr{

    let immutable_buffer: &mut Vec<u8> = &mut vec![];
    buffer.clone_into(immutable_buffer);
    let section_toff = u64::from_le_bytes(immutable_buffer[0x28..0x28 + 8].try_into().expect("Slice with incorrect length"));
    let entry_num_section_t = u16::from_le_bytes(immutable_buffer[0x3C..0x3C + 2].try_into().expect("Slice with incorrect length"));
    let string_index = u16::from_le_bytes(immutable_buffer[0x3E..0x3E + 2].try_into().expect("Slice with incorrect length"));

    let string_table_start = (section_toff + (string_index as u64) * 0x40) as usize;

    let string_table_offset = u64::from_le_bytes(immutable_buffer[string_table_start+0x18..string_table_start + 0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;

    let mut text_: Option<&[u8]> = None;
    let mut text_offset: usize = 0;
    let mut rela_text: Option<&[u8]> = None;
    let mut sym_table: Option<&[u8]> = None;
    let mut fc_offset: usize = 0;

    let mut entry_offset= section_toff as usize;
    for index in 0..entry_num_section_t{
        if index != 1{
            let offset_of_string_name = u32::from_le_bytes(immutable_buffer[entry_offset..entry_offset + 4].try_into().expect("Slice with incorrect length")) as usize;
            let section_name = &immutable_buffer[string_table_offset + offset_of_string_name..];

            if let Some(null_terminator_index) = section_name.iter().position(|&b| b == 0) {
                let section_name_str = std::str::from_utf8(&section_name[..null_terminator_index])
                    .expect("Failed to convert bytes to string");
                match section_name_str {
                    ".text" => {
                        text_offset = u64::from_le_bytes(immutable_buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(immutable_buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        text_ = Some(&immutable_buffer[text_offset..text_offset+section_lenght]);
                    }
                    ".rela.text" => {
                        let section_offset = u64::from_le_bytes(immutable_buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(immutable_buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        rela_text = Some(&immutable_buffer[section_offset..section_offset+section_lenght]);
                    }
                    ".symtab" => {
                        let section_offset = u64::from_le_bytes(immutable_buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(immutable_buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        sym_table = Some(&immutable_buffer[section_offset..section_offset+section_lenght]);
                    }
                    ".rodata.cst8" => {
                        fc_offset = u64::from_le_bytes(immutable_buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                    }
                    _ => {}
                }
            } else {
                println!("No null terminator found in the section name.");
            }
        }
        entry_offset+=0x40;
    }

    if text_ == None {panic!("Text section wasn't found in the byte buffer provided as 'buffer: &[u8]' arguent");}

    let mut symbols = Vec::<&str>::new();
    let fja_offset = text_offset + parse_symbol_table(
        &mut symbols,
        sym_table,
        immutable_buffer,
        string_table_offset
    );

    match rela_text{
        Some(r_text) => {
            let mut entry_offset = 0 as usize;
            while entry_offset < r_text.len(){
                let r_offset = u64::from_le_bytes(r_text[entry_offset..entry_offset + 8].try_into().expect("Slice with incorrect length")) as usize;
                let r_index = (u64::from_le_bytes(r_text[entry_offset+8..entry_offset+16].try_into().expect("Slice with incorrect length"))>>32) as usize;

                let symbol_offset = text_offset+r_offset;
                let offset = resolve_relative_offset(symbol_offset+4, symbols[r_index], fc_offset, buffer_pointer).to_le_bytes();
                buffer[symbol_offset..symbol_offset+4].copy_from_slice(&offset[..4]);
                // println!("Offset: 0x{:x} - Symbol: {}", resolve_relative_offset(symbol_offset+4, symbols[r_index], fc_offset, buffer_pointer), symbols[r_index]);
                entry_offset+=24;
            }
        },
        None =>  {panic!("Relative text section wasn't found in the byte buffer provided as 'buffer: &[u8]' arguent");}
    }

    unsafe{
        return std::mem::transmute::<*mut u8, FunctionPtr>(buffer_pointer.add(fja_offset));
    }
}

