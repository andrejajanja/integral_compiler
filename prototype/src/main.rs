#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]

mod components;
mod stages;


use crate::stages::taylor_ir_compile::optimize_postfix_using_tylor;
use crate::stages::function_lexing::{lex_function, convert_infix_to_postfix};

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}

fn main(){
    // generate_taylor_ir(&String::from("x+9+x+x-x+2*x-x+8"), 1.0, 9);
    let mut sequence = lex_function(&String::from("8-cos(6)/sin(0)"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    println!("{}", temp_str);
}