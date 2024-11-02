use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use crate::components::object_type_definitions::Func;
use std::f64::consts::PI;
use std::process::exit;
use std::ops::{Add, AddAssign, Mul, Index, IndexMut};

/// # Arguments
/// - `poly1` - a first polynomial to be multiplied
/// - `poly2` - a second polynomial to be multiplied
/// 
/// # Result
/// The result of the multiplication is stored in the first argument (`poly1`)
#[derive(Debug, Clone)]
pub struct TsPoly {
    coefs: Vec<f64>,
    offset: f64
}

impl TsPoly{
    const DEFAULT_POW: usize = 30;

    pub fn new() -> TsPoly{
        TsPoly { coefs: vec![0.0; 30], offset: 0.0}
    }

    pub fn from_artifact(fun: Func, offset: f64) -> TsPoly{
        let mut temp = TsPoly { coefs: vec![0.0; TsPoly::DEFAULT_POW], offset: offset};
        
        match fun{
            Func::Sin => TsPoly::generate_sin(&mut temp, offset),
            Func::Cos => TsPoly::generate_cos(&mut temp, offset),
            Func::Tg => todo!(),
            Func::Ctg => todo!(),
            Func::Ln => todo!(),
            Func::Exp => TsPoly::generate_exp(&mut temp, offset),
            Func::Atg => todo!(),
            Func::Actg => todo!(),
            Func::Asin => todo!(),
            Func::Acos => todo!(),
            _ => {
                unrecoverable_error!(
                    "Frontend error | Can't/Shouldn't generate Taylor's polynomial for this Func value",
                    format!("{:?}", fun)
                );
            }
        }

        temp
    }

    //TODO check integrity of this poly
    fn generate_sin(poly: &mut TsPoly, mut offset: f64){
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        let mut fact: f64 = 1.0;
        for i in (0 as usize)..(TsPoly::DEFAULT_POW){
            match i%4 {
                0 => poly.coefs[i] = f64::sin(offset)/fact,
                1 => poly.coefs[i] = f64::cos(offset)/fact,
                2 => poly.coefs[i] = -f64::sin(offset)/fact,
                3 => poly.coefs[i] = -f64::cos(offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
            fact *= i as f64;
        }
    }

    //TODO check integrity of this poly
    fn generate_cos(poly: &mut TsPoly, mut offset: f64){
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        let mut fact: f64 = 1.0;
        for i in (0 as usize)..(TsPoly::DEFAULT_POW){
            match i%4 {
                0 => poly.coefs[i] = f64::cos(offset)/fact,
                1 => poly.coefs[i] = -f64::sin(offset)/fact,
                2 => poly.coefs[i] = -f64::cos(offset)/fact,
                3 => poly.coefs[i] = f64::sin(offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
            fact *= i as f64;
        }
    }

    //TODO check integrity of this poly
    fn generate_exp(poly: &mut TsPoly, offset: f64){
        let mut fact: f64 = 1.0;
        for i in (0 as usize)..(TsPoly::DEFAULT_POW){
            poly.coefs[i] = f64::exp(offset)/fact;
            fact *= i as f64;
        }
    }

    //TODO check integrity of this poly
    fn generate_ln(poly: &mut TsPoly, mut offset: f64){
        //TODO check this value for ln offset
        if offset < 0.5 {unrecoverable_error!("Frontend error | Offset for ln TsPoly is too close to zero", offset);};

        let mut fact: f64 = 1.0;
        let mut coef_fact: f64 = 1.0;

        poly.coefs[0] = f64::ln(offset);
        poly.coefs[1] = 1.0/offset;

        for i in (2 as usize)..(TsPoly::DEFAULT_POW){
            offset*=offset;
            //TODO check integrity of this condition
            if i%2 == 0{
                poly.coefs[i] = coef_fact/offset*fact; //one of these needs a minus in front
            }else{
                poly.coefs[i] = coef_fact/offset*fact; //one of these needs a minus in front
            }

            coef_fact *= (i + 1) as f64; //this is not okay value for factorial
            fact *= (i + 1) as f64;
        }
    }
}

//TODO Finnish these overload operator definitions
impl Add for TsPoly{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for TsPoly{
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}