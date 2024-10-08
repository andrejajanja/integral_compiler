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

struct SymbolEntry<'a>{
    name: &'a str,
    section_offset: usize,
    lenght: usize
}

type FunctionPtr = fn(f64) -> f64;

pub fn find_text_section(buffer: &[u8]) { //-> &[u8]{
    let section_toff = u64::from_le_bytes(buffer[0x28..0x28 + 8].try_into().expect("Slice with incorrect length"));
    let entry_num_section_t = u16::from_le_bytes(buffer[0x3C..0x3C + 2].try_into().expect("Slice with incorrect length"));
    let string_index = u16::from_le_bytes(buffer[0x3E..0x3E + 2].try_into().expect("Slice with incorrect length"));

    let string_table_start = (section_toff + (string_index as u64) * 0x40) as usize;

    let string_table_offset = u64::from_le_bytes(buffer[string_table_start+0x18..string_table_start + 0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;

    let mut text_: Option<&[u8]> = None;
    let mut rela_text: Option<&[u8]> = None;
    let mut sym_table: Option<&[u8]> = None;

    let mut entry_offset= section_toff as usize;
    for index in 0..entry_num_section_t{
        if index != 1{
            let offset_of_string_name = u32::from_le_bytes(buffer[entry_offset..entry_offset + 4].try_into().expect("Slice with incorrect length")) as usize;
            let section_name = &buffer[string_table_offset + offset_of_string_name..];

            if let Some(null_terminator_index) = section_name.iter().position(|&b| b == 0) {
                let section_name_str = std::str::from_utf8(&section_name[..null_terminator_index])
                    .expect("Failed to convert bytes to string");

                match section_name_str {
                    ".text" => {
                        let section_offset = u64::from_le_bytes(buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        text_ = Some(&buffer[section_offset..section_offset+section_lenght]);
                    }
                    ".rela.text" => {
                        let section_offset = u64::from_le_bytes(buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        rela_text = Some(&buffer[section_offset..section_offset+section_lenght]);
                    }
                    ".symtab" => {
                        let section_offset = u64::from_le_bytes(buffer[entry_offset + 0x18..entry_offset +0x18 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        let section_lenght = u64::from_le_bytes(buffer[entry_offset + 0x20..entry_offset +0x20 + 8].try_into().expect("Slice with incorrect length")) as usize;
                        sym_table = Some(&buffer[section_offset..section_offset+section_lenght]);
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
    if rela_text == None {panic!("Relative text section wasn't found in the byte buffer provided as 'buffer: &[u8]' arguent");}

    let mut symbols = Vec::<SymbolEntry>::new();
    //list all symbols:
    match sym_table {
        Some(sym_t) => {
            let mut symbol_offset = 0 as usize;
            while symbol_offset < sym_t.len(){

                let mut symbol_name: &str = "";
            
                let offset_of_string_name = u32::from_le_bytes(sym_t[symbol_offset..symbol_offset + 4].try_into().expect("Slice with incorrect length")) as usize;
                let symbol_name_buffer = &buffer[string_table_offset + offset_of_string_name..];
                if let Some(null_terminator_index) = symbol_name_buffer.iter().position(|&b| b == 0) {
                    symbol_name = std::str::from_utf8(&symbol_name_buffer[..null_terminator_index])
                        .expect("Failed to convert bytes to string");
                }

                let symbol_addr = u64::from_le_bytes(sym_t[symbol_offset+4..symbol_offset+12].try_into().expect("Slice with incorrect length")) as usize;
                let symbol_size = u64::from_le_bytes(sym_t[symbol_offset+12..symbol_offset + 20].try_into().expect("Slice with incorrect length")) as usize;

                println!("{} {} {}", symbol_name, symbol_addr, symbol_size);
                symbol_offset+=24;
            }
        }
        None => {
            panic!("Symbol table wasn't found in the byte buffer provided as 'buffer: &[u8]' arguent");
        }
    }
}

pub unsafe fn link_functions(func_code: *mut u8, buffer_size: usize, called_funcs: Vec<Func>){
    let mut i: usize = 0;
    let mut fn_cnt: usize = 0;

    while i < buffer_size{
        let current_address = func_code.add(i);
        if *current_address == 0xE8{
            let target_address: u64  = match called_funcs[fn_cnt] {
                Func::Sin => {
                    println!("0x{:x}", sin as u64);
                    sin as u64
                },
                Func::Cos => todo!(),
                Func::Tg => tan as u64,
                Func::Ctg => todo!(),
                Func::Ln => todo!(),
                Func::Exp => exp as u64,
                Func::Sqrt => todo!(),
                Func::Atg => todo!(),
                Func::Actg => todo!(),
                Func::Asin => todo!(),
                Func::Acos => todo!(),
                Func::Add => todo!(),
                Func::Sub => todo!(),
                Func::Mul => todo!(),
                Func::Div => todo!(),
                Func::Pow => todo!(),
                Func::Ob => todo!(),
                Func::Cb => todo!(),
                Func::X => todo!(),
                Func::Const => todo!(),
                Func::None => todo!(),
            };

            let next_instruction = (current_address as u64) + 5;
            let difference = target_address as i64 - next_instruction as i64;

            if difference > i32::MAX as i64 || difference < i32::MIN as i64 {
                print!("0x{:x} - {}\n", (func_code as usize) + i, "Diff too big");
            }else{
                print!("0x{:x} - {}\n", (func_code as usize) + i, difference as i32);
            }

            i+=5;
            fn_cnt+=1;
        }else{
            i+=1;
        }
    }

    println!("Start of the code: {:?}\n\n{:?}", func_code, called_funcs);
}

