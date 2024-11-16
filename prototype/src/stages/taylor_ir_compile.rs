#![allow(dead_code)]
use crate::{
    unrecoverable_error,
    components::{
        object_type_definitions::Func, 
        polynomials::TsPoly,
        terminal_decoration::Color,
        taylor_optimizer::optimize_postfix_using_taylor
    },
    stages::function_lexing::{
        convert_infix_to_postfix,
        lex_function
    }
};

pub fn generate_ir_from_taylor_sequence(sequence: &mut Vec<Func>) -> (String, usize) {
    (format!("{:?}", sequence), 1)
}

pub fn generate_taylor_ir(function: &String, precision_center: f64, poly_degre: usize) -> String {
    let mut sequence = lex_function(function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, precision_center, poly_degre);
    let (func_code, ret_addr) = generate_ir_from_taylor_sequence(&mut sequence);
    
    format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1)
}

// let mut temp_str = String::new();
// for elem in sequence {
//     temp_str += &elem.to_string();
//     temp_str += ",";
// }
// println!("{}", temp_str);