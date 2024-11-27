#![allow(dead_code)]
use crate::{
    unrecoverable_error,
    components::{
        object_type_definitions::Func, 
        polynomials::TsPoly,
        terminal_decoration::Color
    }
};

use std::f64::consts::PI;

///value is operand for the unary operator, but &mut sequence[*index-2] is the first operand for binary operation, while value is the second operand
// #[inline(always)]
fn const_handler(operation: Func, sequence: &mut Vec<Func>, value: f64, index: &mut usize, poly_degree: usize){
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
            //TODO Implement domain check for tg
            sequence[*index-1] = Func::Const(value.tan());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Ctg => {
            //TODO Implement domain check for ctg
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
            if !(0.0..=1.0).contains(&value) {
                unrecoverable_error!(
                    "Taylor optimization | Invalid domain error during static const eval, asin on a value outside of domain",
                    format!("{} | {:?}", value, sequence)
                );
            }
            sequence[*index-1] = Func::Const(value.asin());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Acos => {
            if !(0.0..=1.0).contains(&value) {
                unrecoverable_error!(
                    "Taylor optimization | Invalid domain error during static const eval, acos on a value outside of domain",
                    format!("{} | {:?}", value, sequence)
                );
            }
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
            if value < 0.00000001 {
                unrecoverable_error!(
                    "Taylor optimization | Invalid domain error during static const eval, Ln on number smaller than threshold",
                    format!("{} | {:?}", value, sequence)
                );
            }
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
            if value < 0.0 {
                unrecoverable_error!(
                    "Taylor optimization | Invalid domain error during static const eval, sqrt on negative number",
                    format!("{} | {:?}", value, sequence)
                );
            }
            sequence[*index-1] = Func::Const(value.sqrt());
            sequence.remove(*index);
            *index-=1;
        }
        Func::Add => {
            match &mut sequence[*index-2] {
                Func::X => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![value, 1.0], true));
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
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![-value, 1.0], true));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(*value_two-value);
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
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![0.0, value], true));
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
                    if value == 0.0 {
                        //TODO make this sequence print prittier
                        unrecoverable_error!(
                            "Taylor optimization | Devision by 0 error while deviding X with const",
                            format!("{:?}", sequence)
                        );
                    }
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![0.0, 1.0/value], true));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value_two) => {
                    if value == 0.0 {
                        //TODO make this sequence print prittier
                        unrecoverable_error!(
                            "Taylor optimization | Devision by 0 error during static const eval",
                            format!("{:?}", sequence)
                        );
                    }
                    sequence[*index-2] = Func::Const(*value_two/value);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    if value == 0.0 {
                        //TODO make this sequence print prittier
                        unrecoverable_error!(
                            "Taylor optimization error | Devision by 0 error while deviding polynomial with const, sequence state",
                            format!("{:?}", sequence)
                        );
                    }
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
                        sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![1.0], true));
                        sequence.remove(*index);
                        sequence.remove(*index-1);
                        *index-=2;
                    }else if value < TsPoly::DEFAULT_MAX_POW as f64 && value > 0.0 && value.fract() == 0.0 {
                        let mut temp = TsPoly::zero();
                        temp.max_pow = value as usize;
                        temp.coefs[temp.max_pow] = 1.0;
                        temp.truncate(poly_degree);
                        sequence[*index-2] = Func::Poly(temp);
                        sequence.remove(*index);
                        sequence.remove(*index-1);
                        *index-=2;
                    }
                }
                Func::Const(value_two) => {
                    sequence[*index-2] = Func::Const(value_two.powf(value));
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

///X is the first operand, but &mut sequence[[*index-2]] is the first operand for binary operation, while X is the second operand
// #[inline(always)]
fn x_handler(operation: Func, sequence: &mut Vec<Func>, index: &mut usize, precision_center: f64, poly_degree: usize) {
    match operation {
        Func::Sin => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_sin(precision_center, poly_degree, true));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cos => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_cos(precision_center, poly_degree, true));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Tg => todo!("Need to impelment taylor generation for tg"),
        Func::Ctg => todo!("Need to impelment taylor generation for ctg"),
        Func::Sinh => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_sinh(precision_center, poly_degree, true));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Cosh => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_cosh(precision_center, poly_degree, true));
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
            sequence[*index-1] = Func::Poly(TsPoly::generate_ln(precision_center, poly_degree, true));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Exp => {
            sequence[*index-1] = Func::Poly(TsPoly::generate_exp(precision_center, poly_degree, true));
            sequence.remove(*index);
            *index-=1;
        }
        Func::Add => {
            match &mut sequence[*index-2] {
                Func::X => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![0.0, 2.0], true));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![*value, 1.0], true));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) => {
                    poly.coefs[1] += 1.0;
                    if poly.max_pow == 0 {
                        poly.max_pow = 1;
                    }
                    poly.truncate(poly_degree);
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
                    sequence[*index-2] = Func::Const(0.0);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![*value, -1.0], true));
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly) if poly.from_x => {
                    poly.coefs[1] -= 1.0;
                    if poly.max_pow == 1 && poly.coefs[1] == 0.0{
                        poly.max_pow = 0;
                    }
                    poly.truncate(poly_degree);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        //TODO CHECK IF THIS IMPLEMENTATION MAKES MORE SENCE FOR ALL THESE CASES
        Func::Mul => {
            if match &mut sequence[*index-2] {
                Func::X => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![0.0, 0.0, 1.0], true));
                    true
                },
                Func::Const(value) => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![0.0, *value], true));
                    true
                },
                Func::Poly(poly) if poly.from_x => {
                    *poly *= TsPoly::from_vec(vec![0.0, 1.0], true);
                    poly.truncate(poly_degree);
                    true  
                },
                _ => false
            } {
                sequence.remove(*index);
                sequence.remove(*index-1);
                *index-=2;
            }
        }
        Func::Div => {
            if sequence[*index-2] == Func::X{
                sequence[*index-2] = Func::Const(1.0);
                sequence.remove(*index);
                sequence.remove(*index-1);
                *index-=2;
            }
        }
        _ => {}
    }
}

//THIS IS CODE THAT APPLIES ADITIONAL OPTIMIZATION
// if poly.from_x {
//     temp_poly.of(poly.clone()); //FIXME Implement that of method works on a reference, not a value itself
//     sequence[*index-1] = Func::Poly(temp_poly);
//     sequence.remove(*index);
//     *index-=1;
// }else{
//     temp_poly.from_x = false;
//     sequence[*index] = Func::Poly(temp_poly);
// }


///Polynomial is the first operand, but &mut sequence[*index-2] is the first operand for binary operation, while polynomial is the second operand
#[inline(always)]
fn poly_handler(mut poly: TsPoly , operation: Func, sequence: &mut Vec<Func>, index: &mut usize, precision_center: f64, poly_degree: usize){
    match operation {
        Func::Sin => sequence[*index] = Func::Poly(TsPoly::generate_sin(precision_center, poly_degree, false)),
        Func::Cos => sequence[*index] = Func::Poly(TsPoly::generate_cos(precision_center, poly_degree, false)),
        Func::Tg => todo!("Need to impelment taylor generation for tg"),
        Func::Ctg => todo!("Need to impelment taylor generation for ctg"),
        Func::Sinh => sequence[*index] = Func::Poly(TsPoly::generate_sinh(precision_center, poly_degree, false)),
        Func::Cosh => sequence[*index] = Func::Poly(TsPoly::generate_cosh(precision_center, poly_degree, false)),
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
        Func::Ln => sequence[*index] = Func::Poly(TsPoly::generate_ln(precision_center, poly_degree, false)),
        Func::Exp => sequence[*index] = Func::Poly(TsPoly::generate_exp(precision_center, poly_degree, false)),
        Func::Add => {
            match &sequence[*index-2] {
                Func::X if poly.from_x => {
                    poly.coefs[1] += 1.0;
                    if poly.coefs[1] != 0.0 && poly.max_pow == 0 {
                        poly.max_pow = 1;
                    }
                    poly.truncate(poly_degree);
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
                Func::Poly(poly_two) if poly_two.from_x && poly.from_x => {
                    let mut temp = poly+poly_two.clone();
                    temp.truncate(poly_degree);
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Sub => {
            match &sequence[*index-2] {
                Func::X if poly.from_x => {
                    let mut temp = TsPoly::from_vec(vec![0.0, 1.0], true)-poly.clone();
                    temp.truncate(poly_degree);
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Const(value) => {
                    sequence[*index-2] = Func::Poly(TsPoly::from_vec(vec![*value], poly.from_x)-poly.clone());
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                Func::Poly(poly_two) if poly_two.from_x && poly.from_x => {
                    let mut temp = poly_two.clone()-poly.clone();
                    temp.truncate(poly_degree);
                    sequence[*index-2] = Func::Poly(temp);
                    sequence.remove(*index);
                    sequence.remove(*index-1);
                    *index-=2;
                }
                _ => {}
            }
        }
        Func::Mul => {
            match &mut sequence[*index-2] {
                Func::X if poly.from_x => {
                    let mut temp = TsPoly::from_vec(vec![0.0, 1.0], true)*poly.clone();
                    temp.truncate(poly_degree);
                    sequence[*index-2] = Func::Poly(temp);
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
                Func::Poly(poly_two) if poly_two.from_x && poly.from_x => {
                    let mut temp = poly_two.clone()*poly.clone();
                    temp.truncate(poly_degree);
                    sequence[*index-2] = Func::Poly(temp);
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

fn transition_op_handler(operation: Func, sequence: &mut [Func], index: &mut usize, precision_center: f64, poly_degree: usize) {
    match operation {
        Func::Sin => sequence[*index] = Func::Poly(TsPoly::generate_sin(precision_center, poly_degree, false)),
        Func::Cos => sequence[*index] = Func::Poly(TsPoly::generate_cos(precision_center, poly_degree, false)),
        Func::Tg => todo!("Need to impelment taylor generation for tg"),
        Func::Ctg => todo!("Need to impelment taylor generation for ctg"),
        Func::Sinh => sequence[*index] = Func::Poly(TsPoly::generate_sinh(precision_center, poly_degree, false)),
        Func::Cosh => sequence[*index] = Func::Poly(TsPoly::generate_cosh(precision_center, poly_degree, false)),
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
        Func::Ln => sequence[*index] = Func::Poly(TsPoly::generate_ln(precision_center, poly_degree, false)),
        Func::Exp => sequence[*index] = Func::Poly(TsPoly::generate_exp(precision_center, poly_degree, false)),
        Func::Sqrt => todo!("Need to implement handling of SQRT "),
        _ => {}
    }
}

//TODO write detiled description for all component functions in this file
//FIXME Optimize all these clone operations in handler functions
pub fn optimize_postfix_using_taylor(sequence: &mut Vec<Func>, precision_center: f64, poly_degree: usize){
    let mut index: usize = 1;
    while index < sequence.len() {
        let current_elem = sequence[index-1].clone();
        let operation = sequence[index].clone();

        match current_elem {
            Func::X => x_handler(operation, sequence, &mut index, precision_center, poly_degree),
            Func::Const(value) => const_handler(operation, sequence, value, &mut index, poly_degree),
            Func::Poly(poly) => poly_handler(poly, operation, sequence, &mut index, precision_center, poly_degree),
            Func::Div | Func::Pow => transition_op_handler(operation, sequence, &mut index, precision_center, poly_degree),
            _ => {},
        }
        
        index+=1;
    }
}