#![allow(dead_code)]
use crate::{
    components::{object_type_definitions::Func, polynomials::TsPoly, terminal_decoration::Color}, 
    stages::function_parse::{convert_infix_to_postfix, parse_function}
};

use std::f64::consts::PI;

///value is the first operand, but &mut sequence[*index-2] is the first operand for binary operation, while value is the second operand
#[inline(always)]
fn const_handler(operation: Func, sequence: &mut Vec<Func>, value: f64, index: &mut usize){
    match operation {
        Func::Sin => {
            sequence[*index-1] = Func::Const(value.sin());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cos => {
            sequence[*index-1] = Func::Const(value.cos());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tg => {
            sequence[*index-1] = Func::Const(value.tan());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Ctg => {
            sequence[*index-1] = Func::Const(PI/2.0-value.tan());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Sinh => {
            sequence[*index-1] = Func::Const(value.sinh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cosh => {
            sequence[*index-1] = Func::Const(value.cosh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tgh => {
            sequence[*index-1] = Func::Const(value.tanh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Ctgh => {
            sequence[*index-1] = Func::Const(1.0/value.tanh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Atg => {
            sequence[*index-1] = Func::Const(value.atan());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Actg => {
            sequence[*index-1] = Func::Const((1.0/value).atan());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Asin => {
            sequence[*index-1] = Func::Const(value.asin());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Acos => {
            sequence[*index-1] = Func::Const(value.acos());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Arsinh => {
            sequence[*index-1] = Func::Const(value.asinh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Arcosh => {
            sequence[*index-1] = Func::Const(value.acosh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Artgh => {
            sequence[*index-1] = Func::Const(value.atanh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Arctgh => {
            sequence[*index-1] = Func::Const((1.0/value).atanh());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Ln => {
            sequence[*index-1] = Func::Const(value.ln());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Exp => {
            sequence[*index-1] = Func::Const(value.exp());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Sqrt => {
            sequence[*index-1] = Func::Const(value.sqrt());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Add => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[0] = value;
                    temp.coefs[1] = 1.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(value+*value_two);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    poly.coefs[0] += value;
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Sub => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[0] = -value;
                    temp.coefs[1] = 1.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(value-*value_two);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    poly.coefs[0] -= value;
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Mul => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = value;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(*value_two*value);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    *poly *= value;
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Div => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = 1.0/value;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(value / *value_two);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    *poly *= 1.0/value;
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }   
        }
        Func::Pow => {
            match &mut sequence[*index-2] {
                Func::X => {
                    if value == 0.0 {
                        let mut temp = TsPoly::new();
                        temp.max_pow = 1;
                        temp.coefs[0] = 1.0;
                        sequence[*index-2] = Func::Poly(temp);
                        sequence.remove(*index);
                        sequence.remove(*index-1);
                        *index-=2;
                    }else if value < TsPoly::DEFAULT_MAX_POW as f64 && value > 0.0 && value.fract() == 0.0 {
                        let mut temp = TsPoly::new();
                        temp.max_pow = value as usize;
                        temp.coefs[temp.max_pow] = 1.0;
                        sequence[*index-2] = Func::Poly(temp);
                        sequence.remove(*index);
                        sequence.remove(*index-1);
                        *index-=2;
                    }
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(value.powf(*value_two));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        _ => {}
    }
} 

///X is the first operand, but &mut sequence[*index-2] is the first operand for binary operation, while X is the second operand
#[inline(always)]
fn x_handler(operation: Func, sequence: &mut Vec<Func>, index: &mut usize, precision_center: f64, poly_degre: usize) {
    match operation {
        Func::Sin => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_sin(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cos => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_cos(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tg => todo!("Need to impelment taylor generation for tg"),
        Func::Ctg => todo!("Need to impelment taylor generation for ctg"),
        Func::Sinh => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_sinh(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cosh => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_cosh(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tgh => todo!("Need to impelment taylor generation for tgh"),
        Func::Ctgh => todo!("Need to impelment taylor generation for ctgh"),
        Func::Atg => todo!("Need to impelment taylor generation for atg"),
        Func::Actg => todo!("Need to impelment taylor generation for actg"),
        Func::Asin => todo!("Need to impelment taylor generation for asin"),
        Func::Acos => todo!("Need to impelment taylor generation for acos"),
        Func::Arsinh => todo!("Need to impelment taylor generation for asinh"),
        Func::Arcosh => todo!("Need to impelment taylor generation for acosh"),
        Func::Artgh => todo!("Need to impelment taylor generation for actg"),
        Func::Arctgh => todo!("Need to impelment taylor generation for actgh"),
        Func::Ln => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_ln(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Exp => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_exp(precision_center, poly_degre));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Add => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = 2.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[0] = *value;
                    temp.coefs[1] = 1.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    poly.coefs[1] += 1.0;
                    if poly.max_pow == 0 {
                        poly.max_pow = 1;
                    }
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Sub => {
            match &mut sequence[*index-2] {
                Func::X => {
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[0] = *value;
                    temp.coefs[1] = -1.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    poly.coefs[1] -= 1.0;
                    if poly.max_pow == 1 && poly.coefs[1] == 0.0{
                        poly.max_pow = 0;
                    }
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Mul => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 2;
                    temp.coefs[2] = 1.0;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = *value;
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = 1.0;
                    *poly*=temp;
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Div => {
            match &mut sequence[*index-2] {
                Func::X => {
                    sequence[*index-2] = Func::Const(1.0);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

///Polynomial is the first operand, but &mut sequence[*index-2] is the first operand for binary operation, while polynomial is the second operand
#[inline(always)]
fn poly_handler(poly: &mut TsPoly , operation: Func, sequence: &mut Vec<Func>, index: &mut usize, precision_center: f64, poly_degre: usize){
    match operation {
        Func::Sin => {
            let mut temp_poly = TsPoly::generate_sin(precision_center, poly_degre);
            temp_poly.of(poly.clone()); //FIXME Implement that of method works on a reference, not a value itself
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cos => {
            let mut temp_poly = TsPoly::generate_cos(precision_center, poly_degre);
            temp_poly.of(poly.clone());
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tg => todo!("Need to impelment taylor generation for tg"),
        Func::Ctg => todo!("Need to impelment taylor generation for ctg"),
        Func::Sinh => {
            let mut temp_poly = TsPoly::generate_sinh(precision_center, poly_degre);
            temp_poly.of(poly.clone());
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cosh => {
            let mut temp_poly = TsPoly::generate_cosh(precision_center, poly_degre);
            temp_poly.of(poly.clone());
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tgh => todo!("Need to impelment taylor generation for tgh"),
        Func::Ctgh => todo!("Need to impelment taylor generation for ctgh"),
        Func::Atg => todo!("Need to impelment taylor generation for atg"),
        Func::Actg => todo!("Need to impelment taylor generation for actg"),
        Func::Asin => todo!("Need to impelment taylor generation for asin"),
        Func::Acos => todo!("Need to impelment taylor generation for acos"),
        Func::Arsinh => todo!("Need to impelment taylor generation for asinh"),
        Func::Arcosh => todo!("Need to impelment taylor generation for acosh"),
        Func::Artgh => todo!("Need to impelment taylor generation for actg"),
        Func::Arctgh => todo!("Need to impelment taylor generation for actgh"),
        Func::Ln => {
            let mut temp_poly = TsPoly::generate_ln(precision_center, poly_degre);
            temp_poly.of(poly.clone());
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Exp => {
            let mut temp_poly = TsPoly::generate_exp(precision_center, poly_degre);
            temp_poly.of(poly.clone());
            sequence[*index-1] = Func::Poly(temp_poly);
            sequence.remove(*index);
            *index-=1;
        }
        Func::Add => {
            match &sequence[*index-2] {
                Func::X => {
                    poly.coefs[1] += 1.0;
                    if poly.coefs[1] != 0.0 && poly.max_pow == 0 {
                        poly.max_pow = 1;
                    }
                    sequence[*index-2] = Func::Poly(poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    poly.coefs[0] += value;
                    sequence[*index-2] = Func::Poly(poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly_two) => {
                    *poly+=poly_two.clone();
                    sequence[*index-2] = Func::Poly(poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Sub => {
            match &sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.coefs[1] -= 1.0;
                    temp.max_pow = 1;
                    sequence[*index-2] = Func::Poly(temp-poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    let mut temp = TsPoly::new();
                    temp.coefs[0] = *value;
                    sequence[*index-2] = Func::Poly(temp-poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly_two) => {
                    sequence[*index-2] = Func::Poly(poly_two.clone()-poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Mul => {
            match &mut sequence[*index-2] {
                Func::X => {
                    let mut temp = TsPoly::new();
                    temp.max_pow = 1;
                    temp.coefs[1] = 1.0;
                    sequence[*index-2] = Func::Poly(temp*poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    sequence[*index-2] = Func::Poly(*value*poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly_two) => {
                    sequence[*index-2] = Func::Poly(poly_two.clone()*poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

//FIXME Optimize all these clone operations in handler functions
pub fn optimize_postfix_using_tylor(sequence: &mut Vec<Func>, precision_center: f64, poly_degre: usize){
    let mut index: usize = 1;
    while index < sequence.len() {
        let mut current_elem = sequence[index-1].clone();
        let operation = sequence[index].clone();

        if let Func::Const(value) =  current_elem{
            const_handler(operation, sequence, value, &mut index);
        }else if let Func::X = current_elem {
            x_handler(operation, sequence, &mut index, precision_center, poly_degre);
        }else if let Func ::Poly(poly) = &mut current_elem{
            poly_handler(poly, operation, sequence, &mut index, precision_center, poly_degre);
        }
        
        index+=1;
    }
}

pub fn generate_taylor_ir(function: &String, precision_center: f64, poly_degre: usize) -> String {
    let mut sequence = parse_function(function);
    convert_infix_to_postfix(&mut sequence);
    // optimize_postfix_using_tylor(&mut sequence, precision_center, poly_degre);
    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }
    println!("{}", temp_str);

    // format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1)
    String::from("A")
}
