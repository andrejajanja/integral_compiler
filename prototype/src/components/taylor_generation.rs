#![allow(dead_code)]
use super::polynomials::TsPoly;
use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use std::process::exit;
use std::f64::consts::PI;


impl TsPoly {
    pub(crate) fn generate_sin(poly: &mut TsPoly, offset: &mut f64, max_p: usize){
        let multiple = f64::floor(*offset/(2.0*PI));
        *offset-=multiple*2.0*PI;

        
        let mut fact: f64 = 1.0;
        poly.coefs[0] = f64::sin(*offset);
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just ifs and ands
            poly.coefs[i] = match i & 0x3 {
                0 => f64::sin(*offset)/fact,
                1 => f64::cos(*offset)/fact,
                2 => -f64::sin(*offset)/fact,
                3 => -f64::cos(*offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
        }
    }

    pub(crate) fn generate_cos(poly: &mut TsPoly, offset: &mut f64, max_p: usize){
        let multiple = f64::floor(*offset/(2.0*PI));
        *offset-=multiple*2.0*PI;

        poly.coefs[0] = f64::cos(*offset);

        let mut fact: f64 = 1.0;
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just ifs and ands
            poly.coefs[i] = match i%4 {
                0 => f64::cos(*offset)/fact,
                1 => -f64::sin(*offset)/fact,
                2 => -f64::cos(*offset)/fact,
                3 => f64::sin(*offset)/fact,
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_cos - i%4 gives this as a result", num);
                }
            }
        }
    }

    //TODO Make it work
    pub(crate) fn generate_tg(poly: &mut TsPoly, offset: &mut f64, max_p: usize){
        //TODO IMPLEMENT SIGNED POLYNOMIAL AND BETTER RANGE NORMALIZATION FOR OFFSET VARIABLE
        let multiple = f64::floor(*offset/PI);
        *offset-=multiple*PI;

        let mut sin_poly = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p};
        let mut cos_poly = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p};

        let mut fact: f64 = 1.0;
        sin_poly.coefs[0] = f64::sin(*offset);
        cos_poly.coefs[0] = f64::cos(*offset);
        for i in 1..=max_p{
            fact *= i as f64;

            //TODO optimize this match to work faster using just ifs and ands
            match i & 0x3 {
                0 => {
                    sin_poly.coefs[i] = f64::sin(*offset)/fact;
                    cos_poly.coefs[i] = f64::cos(*offset)/fact;
                },
                1 => {
                    sin_poly.coefs[i] = f64::cos(*offset)/fact;
                    cos_poly.coefs[i] = -f64::sin(*offset)/fact;
                },
                2 => {
                    sin_poly.coefs[i] = -f64::sin(*offset)/fact;
                    cos_poly.coefs[i] = -f64::cos(*offset)/fact;
                },
                3 => {
                    sin_poly.coefs[i] = -f64::cos(*offset)/fact;
                    cos_poly.coefs[i] = f64::sin(*offset)/fact;
                },
                num => {
                    unrecoverable_error!("Unforseen error | TsPoly::generate_sin - i%4 gives this as a result", num);
                }
            }
        }

        println!("T(sin) = {}\nT(cos) = {}", sin_poly, cos_poly);
        sin_poly/=cos_poly;

        for i in 0..Self::DEFAULT_MAX_POW{
            poly.coefs[i] = sin_poly.coefs[i];
        }
    }

    pub(crate) fn generate_exp(poly: &mut TsPoly, offset: f64, max_p: usize){
        let mut fact: f64 = 1.0;
        poly.coefs[0] = f64::exp(offset);
        for i in 1..=max_p{
            fact *= i as f64;
            poly.coefs[i] = f64::exp(offset)/fact;
        }
    }

    pub(crate) fn generate_ln(poly: &mut TsPoly, offset: f64, max_p: usize){
        if offset < 0.25 {
            unrecoverable_error!(
                "Frontend error | Can't generate Taylor's polynomial for provided offset value",
                format!("{}  < 0.25", offset)
            );
        }

        poly.coefs[0] = f64::ln(offset);
        if max_p == 0 {return};

        poly.coefs[1] = 1.0/offset;
        if max_p == 1 {return};

        poly.coefs[2] = -1.0/(2.0*offset.powf(2.0));
        for i in 3..=max_p{
            let mut temp = 1.0/(offset.powf(i as f64)*i as f64);
            if i & 0x1 == 0 {
                temp = -temp;
            }
            poly.coefs[i] = temp;
        }
    }

    pub(crate) fn generate_sinh(poly: &mut TsPoly, offset: f64, max_p: usize){
        let exp2x = f64::exp(2.0*offset);
        let sinh_off = (exp2x - 1.0)/exp2x;
        let cosh_off = (exp2x + 1.0)/exp2x;

        poly.coefs[0] = sinh_off;

        let mut fact: f64 = 1.0;
        for i in 1..=max_p{
            fact *= i as f64;
            if i & 0x1 == 0{
                poly.coefs[i] = cosh_off/fact;
            }else{
                poly.coefs[i] = sinh_off/fact;
            }
        }
    }

    pub(crate) fn generate_cosh(poly: &mut TsPoly, offset: f64, max_p: usize){
        let exp2x = f64::exp(2.0*offset);
        let sinh_off = (exp2x - 1.0)/exp2x;
        let cosh_off = (exp2x + 1.0)/exp2x;

        poly.coefs[0] = cosh_off;

        let mut fact: f64 = 1.0;
        for i in 1..=max_p{
            fact *= i as f64;
            if i & 0x1 == 0{
                poly.coefs[i] = sinh_off/fact;
            }else{
                poly.coefs[i] = cosh_off/fact;
            }
        }
    }
}