#![allow(unused_imports)]
use crate::{
    components::{
        object_type_definitions::Func, taylor_optimizer::optimize_postfix_using_taylor, terminal_decoration::Color
    }, stages::function_lexing::{
        convert_infix_to_postfix,
        lex_function
    }, unrecoverable_error
};
use std::process::exit;

//TODO Write description for this function
#[inline(always)]
fn stack_pop_wrapper(stack: &mut Vec<String>) -> String {
    match stack.pop() { 
        Some(value) => value,
        None => { //TODO work on error handling message here
            unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none");
        }
    }
}

pub fn generate_verbose_ir_from_taylor_sequence(sequence: &[Func]) -> String {
    let mut result_stack = Vec::<String>::new();
    let mut generated_poly_addr: i16 = -1;
    let mut fun_code = String::new();

    let mut instrinsic_declarations = String::new();
    let mut declared_instrinsics: u8 = 0; // Bit position/Intrinsic => 1/Pow, 0/Sqrt

    for (index,elem) in sequence.iter().enumerate() {
        match elem {
            Func::Poly(ts_poly) => {
                let temp_code: String; let register: String;

                if ts_poly.from_x {
                    if generated_poly_addr < 0 {
                        (temp_code, register) = ts_poly.generate_ir(None, index as u16);
                        generated_poly_addr = index as i16;
                    }else{
                        (temp_code, register) = ts_poly.generate_ir_from_existing_powers( index as u16, generated_poly_addr as u16);
                    }
                }else{
                    let arg = stack_pop_wrapper(&mut result_stack);
                    (temp_code, register) = ts_poly.generate_ir(Some(arg), index as u16);
                }
                
                result_stack.push(register);
                fun_code+=&format!(";{}. elem\n{}\n",index, temp_code); //fun_code+=&temp_code;
            },
            Func::Add | Func::Sub | Func::Mul | Func::Div=> {
                let arg2 = stack_pop_wrapper(&mut result_stack);
                let arg1 = stack_pop_wrapper(&mut result_stack);

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!(";{}. elem\n{} = {} double {}, {}\n\n", index, temp_arg, elem.ir_string(), arg1, arg2); //fun_code += &format!("{} = f{} double {}, {}", temp_arg, elem, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::Sqrt => {
                let arg = stack_pop_wrapper(&mut result_stack);
                if declared_instrinsics & 1 == 0 {
                    instrinsic_declarations += "declare double @llvm.sqrt.f64(double)";
                    declared_instrinsics |= 1;
                }

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!(";{}. elem\n{} = call double @llvm.sqrt.f64(double {})\n", index, temp_arg, arg);//fun_code += &format!("{} = call double @llvm.sqrt.f64(double {})", temp_arg, arg);
                result_stack.push(temp_arg);
            },
            Func::Pow => {
                let arg2 = stack_pop_wrapper(&mut result_stack);
                let arg1 = stack_pop_wrapper(&mut result_stack);
                if declared_instrinsics & 2 == 0 {
                    instrinsic_declarations += "declare double @llvm.pow.f64(double, double)";
                    declared_instrinsics |= 2;
                }

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!(";{}. elem\n{} = call double @llvm.pow.f64(double {}, double {})\n", index, temp_arg, arg1, arg2);//fun_code += &format!("{} = call double @llvm.pow.f64(double {}, double {})\n", temp_arg, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::X => result_stack.push(String::from("%x")),
            Func::Const(value) => result_stack.push(format!("{:.15e}", value)),
            _ => { unrecoverable_error!("Taylor compilation | Encountered invalid element in provided sequence", elem); }
        }
    }

    let temp_addr = stack_pop_wrapper(&mut result_stack);

    format!("{}\ndefine double @fja(double %x){{\n\n{}ret double {}\n}}", instrinsic_declarations, fun_code, temp_addr)
}

pub fn generate_ir_from_taylor_sequence(sequence: &[Func]) -> String {
    let mut result_stack = Vec::<String>::new();
    let mut generated_poly_addr: i16 = -1;
    let mut fun_code = String::new();

    let mut instrinsic_declarations = String::new();
    let mut declared_instrinsics: u8 = 0; // Bit position/Intrinsic => 1/Pow, 0/Sqrt

    for (index, elem) in sequence.iter().enumerate() {
        match elem {
            Func::Poly(ts_poly) => {
                println!("{}", &ts_poly);
                let temp_code: String; let register: String;
                //println!("{:?}", &ts_poly.coefs);

                if ts_poly.from_x {
                    if generated_poly_addr < 0 {
                        (temp_code, register) = ts_poly.generate_ir(None, index as u16);
                        generated_poly_addr = index as i16;
                    }else{
                        (temp_code, register) = ts_poly.generate_ir_from_existing_powers( index as u16, generated_poly_addr as u16);
                    }
                }else{
                    let arg = stack_pop_wrapper(&mut result_stack);
                    (temp_code, register) = ts_poly.generate_ir(Some(arg), index as u16);
                }
                
                result_stack.push(register);
                fun_code+=&temp_code;
            },
            Func::Add | Func::Sub | Func::Mul | Func::Div=> {
                let arg2 = stack_pop_wrapper(&mut result_stack);
                let arg1 = stack_pop_wrapper(&mut result_stack);

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!("{} = {} double {}, {}\n", temp_arg, elem.ir_string(), arg1, arg2); //fun_code += &format!("{} = f{} double {}, {}", temp_arg, elem, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::Sqrt => {
                let arg = stack_pop_wrapper(&mut result_stack);
                if declared_instrinsics & 1 == 0 {
                    instrinsic_declarations += "declare double @llvm.sqrt.f64(double)";
                    declared_instrinsics |= 1;
                }

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!("{} = call double @llvm.sqrt.f64(double {})\n", temp_arg, arg);//fun_code += &format!("{} = call double @llvm.sqrt.f64(double {})", temp_arg, arg);
                result_stack.push(temp_arg);
            },
            Func::Pow => {
                let arg2 = stack_pop_wrapper(&mut result_stack);
                let arg1 = stack_pop_wrapper(&mut result_stack);
                if declared_instrinsics & 2 == 0 {
                    instrinsic_declarations += "declare double @llvm.pow.f64(double, double)";
                    declared_instrinsics |= 2;
                }

                let temp_arg = format!("%t{}", index as u16);
                fun_code += &format!("{} = call double @llvm.pow.f64(double {}, double {})\n", temp_arg, arg1, arg2);//fun_code += &format!("{} = call double @llvm.pow.f64(double {}, double {})\n", temp_arg, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::X => result_stack.push(String::from("%x")),
            Func::Const(value) => result_stack.push(format!("{:.15e}", value)),
            _ => { unrecoverable_error!("Taylor compilation | Encountered invalid element in provided sequence", elem); }
        }
    }

    let temp_addr = stack_pop_wrapper(&mut result_stack);

    format!("{}\ndefine double @fja(double %x){{\n{}ret double {}\n}}", instrinsic_declarations, fun_code, temp_addr)
}

pub fn generate_taylor_ir(function: &str, precision_center: f64, poly_degre: usize) -> String {
    let mut sequence = lex_function(function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, precision_center, poly_degre);
    // let mut temp_str = String::new();
    // for elem in &sequence {
    //     temp_str += &elem.to_string();
    //     temp_str += ",";
    // }
    // println!("{}", temp_str);
    generate_ir_from_taylor_sequence(&sequence)
}

// if let Func::Poly(poly) = &sequence[0] {
//     println!("{}", poly);
//     let (temp1, temp2) = poly.generate_ir(&mut current_addr);
//     println!("{}\n{}", temp1, temp2);
// }
