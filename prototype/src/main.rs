#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
#![allow(unused_imports)]
mod components;
mod stages;

use std::ptr::NonNull;

use prototype::stages::linking::link_buffer;
use prototype::stages::taylor_ir_compile::generate_taylor_ir;
use prototype::stages::binary_compile::{generate_binary_from_ir, save_generated_binary_to_file, generate_function};

fn main(){
    let fja = generate_function("sin(x)*exp(x)", 1.0, 8);
    println!("Rez: {}", fja(1.0));
    
    //save_generated_binary_to_file(temp, String::from("example.o"));

    // let mut sequence = lex_function(&String::from("8-cos(6)/sin(0)"));
    // convert_infix_to_postfix(&mut sequence);
    // optimize_postfix_using_taylor(&mut sequence, 0.0, 9);
}