#![allow(dead_code)]
use crate::{
    components::object_type_definitions::Func, 
    stages::function_parse::{convert_infix_to_postfix, parse_function}, unrecoverable_error
};

fn optimize_postfix_using_tylor(sequence: &mut Vec<Func>){
    let mut index: usize = 0;

    while index < sequence.len() {
        let current_lexem = sequence[index];

        if matches!(current_lexem, Func::X | Func::Const(_)) {
            continue;
        }

        //I got the idea, just type it out, it's not complicated so far

        match current_lexem {
            Func::Poly(ts_poly) => todo!(),
            Func::Sin => todo!(),
            Func::Cos => todo!(),
            Func::Tg => todo!(),
            Func::Ctg => todo!(),
            Func::Sinh => todo!(),
            Func::Cosh => todo!(),
            Func::Tgh => todo!(),
            Func::Ctgh => todo!(),
            Func::Atg => todo!(),
            Func::Actg => todo!(),
            Func::Asin => todo!(),
            Func::Acos => todo!(),
            Func::Arsinh => todo!(),
            Func::Arcosh => todo!(),
            Func::Artgh => todo!(),
            Func::Arctgh => todo!(),
            Func::Ln => todo!(),
            Func::Exp => todo!(),
            Func::Add => todo!(),
            Func::Sub => todo!(),
            Func::Mul => todo!(),
            Func::Div => todo!(),
            Func::Sqrt => todo!(),
            Func::Pow => todo!(),
            Func::X => continue,
            Func::Const(_) => continue,
            unsupported_lexem => {
                unrecoverable_error!(
                    "Frontend optimization error | Optimizer got the sequence containing unsupported lexem",
                    unsupported_lexem
                );
            }
        }
        index+=1;
    }
}

pub fn generate_taylor_ir(function: &String, _precision_center: f64, _poly_degre: usize) -> String {
    let mut sequence = parse_function(function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    println!("{}", temp_str);

    // format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1)
    String::from("A")
}
