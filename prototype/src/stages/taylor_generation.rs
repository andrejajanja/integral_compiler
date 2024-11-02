#![allow(dead_code)]
use super::polynomials::TsPoly;
use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use std::process::exit;
use std::f64::consts::PI;


impl TsPoly {
    //TODO check integrity of this poly
    pub(crate) fn generate_sin(poly: &mut TsPoly, mut offset: f64){
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        let mut fact: f64 = 1.0;
        for i in 0..TsPoly::DEFAULT_POW{
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
    pub(crate) fn generate_cos(poly: &mut TsPoly, mut offset: f64){
        let multiple = f64::floor(offset/(2.0*PI));
        offset-=multiple*2.0*PI;

        let mut fact: f64 = 1.0;
        for i in 0..TsPoly::DEFAULT_POW{
            match i%4 {
                0 => poly.coefs[i] = f64::cos(offset)/fact,
                1 => poly.coefs[i] = -f64::sin(offset)/fact,
                2 => poly.coefs[i] = -f64::cos(offset)/fact,
                3 => poly.coefs[i] = f64::sin(offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_cos - i%4 gives this as a result", num);
                }
            }
            fact *= i as f64;
        }
    }

    //TODO check integrity of this poly
    pub(crate) fn generate_exp(poly: &mut TsPoly, offset: f64){
        let mut fact: f64 = 1.0;
        for i in 0..TsPoly::DEFAULT_POW{
            poly.coefs[i] = f64::exp(offset)/fact;
            fact *= i as f64;
        }
    }

    //TODO check integrity of this poly
    pub(crate) fn generate_ln(poly: &mut TsPoly, mut offset: f64){
        //TODO check this value for ln offset
        if offset < 0.5 {
            unrecoverable_error!("Frontend error | Offset arg in TsPoly::generate_ln is too close to zero", offset);
        };

        let mut fact: f64 = 2.0;
        let mut coef_fact: f64 = 1.0;

        poly.coefs[0] = f64::ln(offset);
        poly.coefs[1] = 1.0/offset;

        for i in (2 as usize)..(TsPoly::DEFAULT_POW){
            offset*=offset;
            let mut temp = coef_fact/(offset*fact);

            if i%2 == 0{
                temp = -temp;
            }

            poly.coefs[i] = temp;

            coef_fact *= i as f64;
            fact *= (i + 1) as f64;
        }
    }
}