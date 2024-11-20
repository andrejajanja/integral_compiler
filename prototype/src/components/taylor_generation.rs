#![allow(dead_code, unused_imports)]
use super::polynomials::TsPoly;
use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use std::process::exit;
use std::f64::consts::PI;


impl TsPoly {
    pub fn generate_sin(mut offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        
        let mut fact: f64 = 1.0;
        temp.coefs[0] = f64::sin(offset);
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just lookup table
            temp.coefs[i] = match i & 0x3 {
                0 => f64::sin(offset)/fact,
                1 => f64::cos(offset)/fact,
                2 => -f64::sin(offset)/fact,
                3 => -f64::cos(offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
        }

        temp.put_offset(offset);
        temp
    }

    pub fn generate_cos(mut offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        
        let mut fact: f64 = 1.0;
        temp.coefs[0] = f64::sin(offset);
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just lookup table
            temp.coefs[i] = match i & 0x3 {
                0 => f64::cos(offset)/fact,
                1 => -f64::sin(offset)/fact,
                2 => -f64::cos(offset)/fact,
                3 => f64::sin(offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
        }

        temp.put_offset(offset);
        temp
    }

    pub fn generate_tg_parts(mut offset: f64, max_p: usize, from_x: bool) -> (TsPoly, TsPoly){
        let multiple = f64::floor(offset/PI);
        offset-=multiple*PI;

        let mut sin_poly = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let mut cos_poly = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};

        let mut fact: f64 = 1.0;
        sin_poly.coefs[0] = f64::sin(offset);
        cos_poly.coefs[0] = f64::cos(offset);
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just ifs and ands
            match i & 0x3 {
                0 => {
                    sin_poly.coefs[i] = f64::sin(offset)/fact;
                    cos_poly.coefs[i] = f64::cos(offset)/fact;
                },
                1 => {
                    sin_poly.coefs[i] = f64::cos(offset)/fact;
                    cos_poly.coefs[i] = -f64::sin(offset)/fact;
                },
                2 => {
                    sin_poly.coefs[i] = -f64::sin(offset)/fact;
                    cos_poly.coefs[i] = -f64::cos(offset)/fact;
                },
                3 => {
                    sin_poly.coefs[i] = -f64::cos(offset)/fact;
                    cos_poly.coefs[i] = f64::sin(offset)/fact;
                },
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
        }

        sin_poly.put_offset(offset);
        cos_poly.put_offset(offset);
        (sin_poly, cos_poly)
    }

    pub fn generate_exp(offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let mut fact: f64 = 1.0;
        temp.coefs[0] = f64::exp(offset);
        for i in 1..=max_p{
            fact *= i as f64;
            temp.coefs[i] = f64::exp(offset)/fact;
        }
        temp.put_offset(offset);
        temp
    }

    pub fn generate_ln(offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        if offset < 0.25 {
            unrecoverable_error!(
                "Frontend error | Can't generate Taylor's polynomial for provided offset value",
                format!("{}  < 0.25", offset)
            );
        }
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        temp.coefs[0] = f64::ln(offset);
        if max_p == 0 {return temp};

        temp.coefs[1] = 1.0/offset;
        if max_p == 1 {
            temp.put_offset(offset);
            return temp;
        }

        temp.coefs[2] = -1.0/(2.0*offset.powf(2.0));
        for i in 3..=max_p{
            let mut temp_value = 1.0/(offset.powf(i as f64)*i as f64);
            if i & 0x1 == 0 {
                temp_value = -temp_value;
            }
            temp.coefs[i] = temp_value;
        }

        temp.put_offset(offset);
        temp
    }

    pub fn generate_sinh(offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let exp2x = f64::exp(2.0*offset);
        let sinh_off = (exp2x - 1.0)/exp2x;
        let cosh_off = (exp2x + 1.0)/exp2x;

        temp.coefs[0] = sinh_off;

        let mut fact: f64 = 1.0;
        for i in 1..=max_p{
            fact *= i as f64;
            if i & 0x1 == 0{
                temp.coefs[i] = cosh_off/fact;
            }else{
                temp.coefs[i] = sinh_off/fact;
            }
        }

        temp.put_offset(offset);
        temp
    }

    pub fn generate_cosh(offset: f64, max_p: usize, from_x: bool) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p, from_x: from_x};
        let exp2x = f64::exp(2.0*offset);
        let sinh_off = (exp2x - 1.0)/exp2x;
        let cosh_off = (exp2x + 1.0)/exp2x;

        temp.coefs[0] = cosh_off;

        let mut fact: f64 = 1.0;
        for i in 1..=max_p{
            fact *= i as f64;
            if i & 0x1 == 0{
                temp.coefs[i] = sinh_off/fact;
            }else{
                temp.coefs[i] = cosh_off/fact;
            }
        }

        temp.put_offset(offset);
        temp
    }

}


// Func::Add => {
//     match &mut sequence[index-2] {
//         Func::X => {
//             let mut temp = TsPoly::new();
//             temp.max_pow = 1;
//             temp.coefs[0] = value;
//             temp.coefs[1] = 1.0;
//             sequence[index-2] = Func::Poly(temp);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Const(value_two) => {
//             sequence[index-2] = Func::Const(value+*value_two);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Poly(poly) => {
//             poly.coefs[0] += value;
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         _ => {}
//     }
// }
// Func::Sub => {
//     match &mut sequence[index-2] {
//         Func::X => {
//             let mut temp = TsPoly::new();
//             temp.max_pow = 1;
//             temp.coefs[0] = -value;
//             temp.coefs[1] = 1.0;
//             sequence[index-2] = Func::Poly(temp);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Const(value_two) => {
//             sequence[index-2] = Func::Const(value-*value_two);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Poly(poly) => {
//             poly.coefs[0] -= value;
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         _ => {}
//     }
// }
// Func::Mul => {
//     match &mut sequence[index-2] {
//         Func::X => {
//             let mut temp = TsPoly::new();
//             temp.max_pow = 1;
//             temp.coefs[1] = value;
//             sequence[index-2] = Func::Poly(temp);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Const(value_two) => {
//             sequence[index-2] = Func::Const(*value_two*value);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Poly(poly) => {
//             *poly *= value;
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         _ => {}
//     }
// }
// Func::Div => {
//     match &mut sequence[index-2] {
//         Func::X => {
//             let mut temp = TsPoly::new();
//             temp.max_pow = 1;
//             temp.coefs[1] = 1.0/value;
//             sequence[index-2] = Func::Poly(temp);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Const(value_two) => {
//             sequence[index-2] = Func::Const(*value_two/value);
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         Func::Poly(poly) => {
//             *poly *= 1.0/value;
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         _ => {}
//     }   
// }
// Func::Pow => {
//     match &mut sequence[index-2] {
//         Func::X => {
//             if value == 0.0 {
//                 let mut temp = TsPoly::new();
//                 temp.max_pow = 1;
//                 temp.coefs[0] = 1.0;
//                 sequence[index-2] = Func::Poly(temp);
//                 sequence.remove(index);
//                 sequence.remove(index-1);
//                 index-=2;
//             }else if value < TsPoly::DEFAULT_MAX_POW as f64 && value > 0.0 && value.fract() == 0.0 {
//                 let mut temp = TsPoly::new();
//                 temp.max_pow = value as usize;
//                 temp.coefs[value as usize] = 1.0;
//                 sequence[index-2] = Func::Poly(temp);
//                 sequence.remove(index);
//                 sequence.remove(index-1);
//                 index-=2;
//             }
//         }
//         Func::Const(value_two) => {
//             sequence[index-2] = Func::Const(value.powf(*value_two));
//             sequence.remove(index);
//             sequence.remove(index-1);
//             index-=2;
//         }
//         _ => {}
//     }
// }
