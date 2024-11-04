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
            poly.coefs[i] = match i%4 {
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

    pub(crate) fn generate_exp(poly: &mut TsPoly, offset: f64, max_p: usize){
        let mut fact: f64 = 1.0;
        poly.coefs[0] = f64::exp(offset);
        for i in 1..=max_p{
            fact *= i as f64;
            poly.coefs[i] = f64::exp(offset)/fact;
        }
    }

}