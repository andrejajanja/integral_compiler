#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
mod components;
mod stages;

use prototype::stages::taylor_ir_compile::generate_taylor_ir;

extern "C" { 
    static __code_buffer: u8; // Start of the reserved block, size is 16KB
}

fn main(){
    let _temp = generate_taylor_ir(&String::from("cos(x+1)*e^x+sin(x)*ln(x)"), 1.0, 6);
    //println!("{}", );
    // let mut sequence = lex_function(&String::from("8-cos(6)/sin(0)"));
    // convert_infix_to_postfix(&mut sequence);
    // optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    // let mut temp_str = String::new();
    // for elem in sequence {
    //     temp_str += &elem.to_string();
    //     temp_str += ",";
    // }

    // println!("{}", temp_str);
}