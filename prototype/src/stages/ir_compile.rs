#![allow(dead_code, unused_imports)]
use crate::unrecoverable_error;
use crate::components::{
    object_type_definitions::*,
    terminal_decoration::Color,
    auxilary_functions::safely_pop_from_stacks
};
use crate::stages::function_lexing::{lex_function, convert_infix_to_postfix};
use std::process::exit;

fn generate_ir_from_postfix(mut elems: Vec<Func>) -> (String,Vec<Func>, i16){
    let mut unique_funcs: Vec<Func> = Vec::<Func>::new();
    let mut code = String::from("");

    let mut address: i16 = 0;
    let mut operand_stack: Vec<i16> = Vec::<i16>::new();
    let mut const_stack: Vec<String> = Vec::<String>::new();

    while !elems.is_empty() {
        let temp = elems.remove(0);

        //determining if op should be added to the list of ones to be declared beforehand
        if !(unique_funcs.contains(&temp) || matches!(&temp, Func::Const(_) | Func::X | Func::Add | Func::Sub | Func::Mul | Func::Div)){
            unique_funcs.push(temp.clone());
        }

        match &temp{
            //defining the LLVM IR code output for UNARY ops:
            Func::Sqrt | Func::Ln | Func::Exp | Func::Sin | Func::Cos | Func::Tg | Func::Ctg | Func::Asin | Func::Acos | Func::Atg | Func::Actg=> {
                let oper: String = safely_pop_from_stacks(&mut operand_stack, &mut const_stack, true);
                address+=1;
                code += &format!("\t%{} = call double @{}(double {}) nounwind\n", address, temp.ir_string(), oper);
                match temp {
                    Func::Ctg => {
                        address+=1;
                        code += &format!("\t%{} = fdiv double 1.0, {}\n", address, address-1);
                    }
                    Func::Actg => {
                        address+=1;
                        code += &format!("\t%{} = fsub double pi_over_2, {}\n", address, address-1);
                    }
                    _ => {}
                }
                operand_stack.push(address);
            },
            //defining the LLVM IR code output for BINARY ops:
            Func::Add | Func::Sub | Func::Mul | Func::Div | Func::Pow => {
                let first_oper: String = safely_pop_from_stacks(&mut operand_stack, &mut const_stack, false);
                let second_oper: String = safely_pop_from_stacks(&mut operand_stack, &mut const_stack, false);

                address+=1;
                code += &format!("\t%{} = {} double {}, {}\n", address, temp.ir_string(), first_oper, second_oper); 
                
                operand_stack.push(address);
            },

            //X and Const implementations:
            Func::X => operand_stack.push(0),
            Func::Const(value) => {
                const_stack.push(format!("{:.6e}", value));
                operand_stack.push(-1);     
            },
            _ => {
                unrecoverable_error!(
                    "Frontend error | During compiling of postfix form",
                    format!("Failed to compile function due unsupported node type '{}', in postfix form.", temp)
                );
            }
        }
    }

    (code, unique_funcs, address-1)
}

pub fn generate_ir(function: &str) -> String {
    let mut function_collection = lex_function(function);
    convert_infix_to_postfix(&mut function_collection);

    let (mut func_code,functions_to_define, ret_addr) = generate_ir_from_postfix(function_collection);
    let mut code = String::from("");

    for elem in functions_to_define {
        code += &format!("declare double @{}(double) nounwind\n", elem.ir_string());
        if elem == Func::Actg {
            func_code = "\t%pi_over_2 = fpext double 1, double\n\n".to_owned() + &func_code;
        }
    }

    code += &format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1);
    code
}
