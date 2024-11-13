#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]

mod components;
mod stages;

use crate::stages::{
    taylor_ir_compile::optimize_postfix_using_tylor,
    function_parse::{parse_function,convert_infix_to_postfix}
};

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}
fn main(){

    let function = String::from("exp(9)");
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    println!("{temp_str}");
    // generate_taylor_ir(&String::from("x+9+x+x-x+2*x-x+8"), 1.0, 9);
    // let mut a = TsPoly::from_func(Func::Exp, 1.5, 8);
    // let b = TsPoly::from_func(Func::Sin, 1.5, 8);
    // a.of(b);
    // a.truncate(10);
    // println!("y = {}", a);
}

#[cfg(test)]
mod tests {
    mod parsing_and_postfix;
    mod taylor_optimizer;
}