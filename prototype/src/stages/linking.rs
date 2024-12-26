#![allow(unused_imports)]
use crate::{
    unrecoverable_error,
    components::external_functions::*,
    components::terminal_decoration::Color
};

use std::{
    process::exit,
    ptr::NonNull
};

pub type FunctionType = fn(f64, *mut f64) -> f64;

fn parse_symbol_table<'a>(symbols: &mut Vec<&'a str>, sym_table: Option<&[u8]>, object_file_buffer: &'a [u8], string_table_start: usize) -> usize{
    let mut temp_fja_offset: u64 = u64::MAX;

    if let Some(sym_t) = sym_table {
        let mut entry_offset: usize = 0;
        while entry_offset < sym_t.len(){

            let offset_of_string_name = u32::from_le_bytes(sym_t[entry_offset..entry_offset + 4].try_into().expect("Slice with incorrect length")) as usize;
            let symbol_name_buffer = &object_file_buffer[string_table_start + offset_of_string_name..];

            let symbol_name: &str = if let Some(null_terminator_index) = symbol_name_buffer.iter().position(|&b| b == 0) {
                unsafe { std::str::from_utf8_unchecked(&symbol_name_buffer[..null_terminator_index]) }
            }else {
                ""
            };

            if symbol_name == "fja" {
                temp_fja_offset = u64::from_le_bytes(sym_t[(entry_offset+8)..(entry_offset+16)].try_into().expect("Slice with incorrect length"));
            }

            symbols.push(symbol_name);
            entry_offset+=24;
        }

        if temp_fja_offset == u64::MAX {
            unrecoverable_error!("Linker error | Parsing of the symbol table", "'fja' symbol wasn't found in the symbol table");
        }

        temp_fja_offset as usize
    }else{
        unrecoverable_error!("Linker error | Parsing of the symbol table", "Symbol table wasn't found in the byte buffer provided");
    }
}

fn resolve_relative_offset(symbol_offset: usize, symbol_name: &str, fc_offset: usize, buffer_ptr: *mut u8) -> i32{
    if symbol_name.contains(".LCPI") {
        if let Some(pos) = symbol_name.find('_') {
            match &symbol_name[pos + 1..].parse::<i32>() {
                Ok(value) => {
                    return ((fc_offset as i32) + value*8) - (symbol_offset as i32);
                },
                Err(e) => {unrecoverable_error!("Linker Error | failed to parse usize during the resolve of relative offset", e);}
            }
        }
    }

    let pointer_addr: usize = unsafe { buffer_ptr.add(symbol_offset) as usize};
    (match symbol_name{
        "sin" => {(sin as usize).wrapping_sub(pointer_addr)},
        "cos" => {(cos as usize).wrapping_sub(pointer_addr)},
        "tan" => {(tan as usize).wrapping_sub(pointer_addr)},
        "exp" => {(exp as usize).wrapping_sub(pointer_addr)},
        "ln" => {(ln as usize).wrapping_sub(pointer_addr)},
        "asin" => {(asin as usize).wrapping_sub(pointer_addr)},
        "acos" => {(acos as usize).wrapping_sub(pointer_addr)},
        "atan" => {(atan as usize).wrapping_sub(pointer_addr)},
        "sqrt" => {(sqrt as usize).wrapping_sub(pointer_addr)},
        _ => {unrecoverable_error!("Linker Error | Unrecognized symbol in the external functions table", symbol_name);}
    }) as i32
}

pub fn link_buffer(buffer: &mut[u8], buffer_ptr: NonNull<u8>) -> FunctionType{
    let immutable_buffer: &mut Vec<u8> = &mut Vec::<u8>::new();
    buffer.clone_into(immutable_buffer);
    let section_toff = u64::from_le_bytes(immutable_buffer[0x28..0x28 + 8].try_into().expect("Slice with incorrect length"));
    let entry_num_section_t = u16::from_le_bytes(immutable_buffer[0x3C..0x3C + 2].try_into().expect("Slice with incorrect length"));
    let string_index = u16::from_le_bytes(immutable_buffer[0x3E..0x3E + 2].try_into().expect("Slice with incorrect length"));

    let string_table_start = (section_toff + (string_index as u64) * 0x40) as usize;

    let string_table_offset = u64::from_le_bytes(immutable_buffer[string_table_start+0x18..string_table_start + 0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;

    let mut text_section: Option<&[u8]> = None;
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
                        text_section = Some(&immutable_buffer[text_offset..text_offset+section_lenght]);
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
                unrecoverable_error!("Linker Error | During analysis of ELF headers", "No null terminator found in the section name, check ELF byte buffer");
            }
        }
        entry_offset+=0x40;
    }

    if text_section.is_none() {
        unrecoverable_error!("Linker Error | Invalid result of ELF headers analisys", "Text section wasn't found in ELF byte buffer");
    }

    let mut symbols = Vec::<&str>::new();
    let fja_offset = text_offset + parse_symbol_table(
        &mut symbols,
        sym_table,
        immutable_buffer,
        string_table_offset
    );

    let raw_buffer_ptr: *mut u8 = buffer_ptr.as_ptr();

    if let Some(r_text) = rela_text {
        let mut entry_offset: usize = 0;
        while entry_offset < r_text.len(){
            let r_offset = u64::from_le_bytes(r_text[entry_offset..entry_offset + 8].try_into().expect("Slice with incorrect length")) as usize;
            let r_index = (u64::from_le_bytes(r_text[entry_offset+8..entry_offset+16].try_into().expect("Slice with incorrect length"))>>32) as usize;

            let symbol_offset = text_offset+r_offset;
            let offset = resolve_relative_offset(symbol_offset+4, symbols[r_index], fc_offset, raw_buffer_ptr).to_le_bytes();
            buffer[symbol_offset..symbol_offset+4].copy_from_slice(&offset[..4]);
            entry_offset+=24;
        }
    }
    //else{
    //     unrecoverable_error!("Linker Error | Invalid result of ELF headers analisys", "Relative text section wasn't found in the ELF byte buffer");
    // }

    unsafe{
        std::mem::transmute::<*mut u8, FunctionType>(raw_buffer_ptr.add(fja_offset))
    }
}

