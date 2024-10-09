#![allow(dead_code)]
use std::process::exit;//, fs::read_to_string, fs::File, io::Write};
use crate::parts::object_type_definitions::*;
use crate::stages::string_to_tree_iterative::{string_to_vec_of_node, vec_infix_to_postfix};

fn compile_postfix(mut elems: Vec<Node>) -> (String,Vec<Func>, i16){
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
                                println!("ERROR:\nNo constant on the const_stack, even though at least one was expected to be.");
                                exit(0);
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
                    println!("ERROR:\nNo operands on the stack, even though at least one was expected to be.");
                }else{
                    println!("ERROR:\nNo operands on the stack, even though at least two were expected to be.");
                }
    
                exit(0);
            }
        }
    }

    let mut unique_funcs: Vec<Func> = Vec::<Func>::new();
    let mut code = String::from("");

    let mut address: i16 = 0;
    let mut operand_stack: Vec<i16> = Vec::<i16>::new();
    let mut const_stack: Vec<String> = Vec::<String>::new();

    let not_unique_funcs: Vec<Func> = vec![Func::Const, Func::X, Func::Add, Func::Sub, Func::Mul, Func::Div];

    while !elems.is_empty() {
        let temp = elems.remove(0);

        //determining if op should be added to the list of ones to be declared beforehand
        if !(unique_funcs.contains(&temp.op) || not_unique_funcs.contains(&temp.op)){ //more efficient if :D
            unique_funcs.push(temp.op);
        }

        match &temp.op{
            //defining the LLVM IR code output for UNARY ops:
            Func::Sqrt | Func::Ln | Func::Exp | Func::Sin | Func::Cos | Func::Tg | Func::Ctg | Func::Asin | Func::Acos | Func::Atg | Func::Actg=> {
                let oper: String = safely_pop_from_stacks(&mut operand_stack, &mut const_stack, true);
                address+=1;
                code += &format!("\t%{} = call double @{}(double {}) nounwind\n", address, temp.op.ir_string(), oper);
                match temp.op {
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
                code += &format!("\t%{} = {} double {}, {}\n", address, temp.op.ir_string(), first_oper, second_oper); 
                
                operand_stack.push(address);
            },

            //X and Const implementations:
            Func::X => {
                operand_stack.push(0)
            },
            Func::Const => {
                match temp.c {
                    Some(c) => {
                        const_stack.push(format!("{:.6e}", c));
                        operand_stack.push(-1);
                    }
                    None => {
                        println!("ERROR: Logical error occured, Node is of op type 'Const', but c is None.");
                        exit(0);
                    }
                }            
            },
            
            _ => {
                println!("ERROR: Failed to compile function due unsupported node type '{}', in postfix form.", temp.op.to_string());
                exit(0);
            }
        }    
    }

    (code, unique_funcs, address-1)
}

pub fn generate_ir(function: &String) -> String {
    let function_infix = string_to_vec_of_node(function);
    let function_postfix = vec_infix_to_postfix(function_infix);

    let (mut func_code,functions_to_define, ret_addr) = compile_postfix(function_postfix);
    let mut code = String::from("");

    for elem in functions_to_define {
        code += &format!("declare double @{}(double) nounwind\n", elem.ir_string());
        if elem == Func::Actg {
            func_code = "\t%pi_over_2 = fpext double 1, double\n\n".to_owned() + &func_code;
        }
    }

    code += "\ndefine double @fja(double %x){\n";
    code += &func_code;
    code += &("\tret double %".to_owned() + &(ret_addr+1).to_string() + "\n}");

    code
}

