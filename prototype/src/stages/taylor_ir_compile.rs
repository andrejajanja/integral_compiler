#![allow(dead_code)]
use crate::unrecoverable_error;
use std::process::exit;
use crate::components::{
    object_type_definitions::*,
    terminal_decoration::Color,
    polynomials::TsPoly
};

use crate::stages::function_parse_iterative::{parse_function, convert_infix_to_postfix};

enum TsOps{
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Of
}

fn safely_pop_from_stacks(op_st: &mut Vec<i16>, cnst_st: &mut Vec<String>, one_two: bool) -> String{
    match op_st.pop() {
        Some(x) => {
            let temp: String;
            match &x {
                -1 => {
                    match cnst_st.pop() {
                        Some(cnst) => {
                            temp = cnst.clone();
                        },
                        None => {
                            unrecoverable_error!("Frontend error | During compiling of postfix form", "No constant on the const_stack, even though at least one was expected to be.");
                        }
                    }
                },
                0 => {
                    temp = "%x".to_owned();
                },
                _ => {
                    temp = "%".to_owned() + &x.to_string();
                }
            }
            temp
        },
        None => {
            if one_two {
                unrecoverable_error!("Frontend error | During compiling of postfix form", "No operands on the stack, even though at least one was expected to be.");
            }else{
                unrecoverable_error!("Frontend error | During compiling of postfix form", "No operands on the stack, even though at least two was expected to be.");
            }
        }
    }
}

fn generate_taylor_sequence() -> (Vec<TsPoly>, Vec<TsOps>){
    let mut polys = Vec::<TsPoly>::new();
    let mut binary_ops = Vec::<TsOps>::new();


    (polys, binary_ops)
}

fn taylor_compile_postfix(mut elems: Vec<Node>) -> (String, i16){
    let mut code = String::from("");    

    let mut address: i16 = 0;
    let mut operand_stack: Vec<i16> = Vec::<i16>::new();
    let mut const_stack: Vec<String> = Vec::<String>::new();
    
    

    (code, address-1)
}

pub fn generate_taylor_ir(function: &String) -> String {
    let function_infix = parse_function(function);
    let function_postfix = convert_infix_to_postfix(function_infix);

    let (func_code, ret_addr) = taylor_compile_postfix(function_postfix);

    format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1)
}
