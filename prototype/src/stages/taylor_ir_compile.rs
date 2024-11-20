#![allow(dead_code)]
use crate::{
    components::{
        object_type_definitions::Func, polynomials::TsPoly, taylor_optimizer::optimize_postfix_using_taylor, terminal_decoration::Color
        //auxilary_functions::safely_pop_from_stacks
    }, stages::function_lexing::{
        convert_infix_to_postfix,
        lex_function
    }, unrecoverable_error
};

pub fn generate_ir_from_taylor_sequence(sequence: &Vec<Func>) -> String {
    let mut result_stack = Vec::<String>::new();
    let mut generated_poly_addr: i16 = -1;
    let mut current_addr = 0;
    let mut fun_code = String::new();

    let mut instrinsics_declaration = String::new();
    let mut declared_instrinsics: u8 = 0;

    for elem in sequence {
        match elem {
            //FIXME Add here the condition check if polynomial is on x or some other virtual register, thus a value should be poped from stack
            Func::Poly(ts_poly) => {
                let temp_code: String; let register: String;

                if ts_poly.from_x {
                    if generated_poly_addr < 0 {
                        (temp_code, register) = ts_poly.generate_ir_from_existing_powers( current_addr, generated_poly_addr as u16);
                    }else{
                        (temp_code, register) = ts_poly.generate_ir(None, current_addr);
                        generated_poly_addr = current_addr as i16;
                    }
                }else{
                    let arg = match result_stack.pop() { Some(value) => value,
                        None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                    };
                    (temp_code, register) = ts_poly.generate_ir(Some(arg), current_addr);
                }
                

                result_stack.push(register);
                fun_code+=&temp_code;
            },
            Func::Add | Func::Sub | Func::Mul | Func::Div=> {
                let arg2 = match result_stack.pop() { Some(value) => value,
                    None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                };
                let arg1 = match result_stack.pop() { Some(value) => value, 
                    None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                };

                let temp_arg = format!("%t{}", current_addr);
                fun_code += &format!("{} = f{} double {}, {}", temp_arg, elem, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::Sqrt => {
                let arg = match result_stack.pop() { Some(value) => value,
                    None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                };

                if declared_instrinsics&1 == 0 {
                    instrinsics_declaration += "declare double @llvm.sqrt.f64(double)";
                    declared_instrinsics |= 1;
                }

                let temp_arg = format!("%t{}", current_addr);
                fun_code += &format!("{} = call double @llvm.sqrt.f64(double {})", temp_arg, arg);
                result_stack.push(temp_arg);
            },
            Func::Pow => {
                let arg2 = match result_stack.pop() { Some(value) => value,
                    None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                };
                let arg1 = match result_stack.pop() { Some(value) => value, 
                    None => { unrecoverable_error!("Taylor compilation | Expected an argument on stack", "found none"); }
                };

                if declared_instrinsics&2 == 0 {
                    instrinsics_declaration += "declare double @llvm.pow.f64(double, double)";
                    declared_instrinsics |= 2;
                }

                let temp_arg = format!("%t{}", current_addr);
                fun_code += &format!("{} = call double @llvm.pow.f64(double {}, double {})", temp_arg, arg1, arg2);
                result_stack.push(temp_arg);
            },
            Func::X => result_stack.push(String::from("%x")),
            Func::Const(value) => result_stack.push(format!("{:.15e}", value)),
            _ => {unrecoverable_error!("Taylor compilation | Encountered invalid element in provided sequence", elem);}
        }
        current_addr+=1;
    }

    format!("{}\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", instrinsics_declaration, fun_code, 1.to_string())
}

pub fn generate_taylor_ir(function: &String, precision_center: f64, poly_degre: usize) -> String {
    let mut sequence = lex_function(function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, precision_center, poly_degre);

    let mut temp_str = String::new();
    for elem in &sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }
    println!("{}", temp_str);
    // generate_ir_from_taylor_sequence(&mut sequence)
    String::from("AA")
}

// if let Func::Poly(poly) = &sequence[0] {
//     println!("{}", poly);
//     let (temp1, temp2) = poly.generate_ir(&mut current_addr);
//     println!("{}\n{}", temp1, temp2);
// }
