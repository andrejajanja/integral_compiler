#![allow(unused_imports)]
use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use crate::components::object_type_definitions::Func;
use std::process::exit;

//TODO write description for everything defined for this struct

/// Struct for handling, generating and manipulating [Taylor polynomials](https://en.wikipedia.org/wiki/Taylor_series)
/// # Description-Fields
/// # Overloaded operators
#[derive(Debug, Clone, PartialEq)]
pub struct TsPoly {
    pub coefs: Vec<f64>,
    pub max_pow: usize
}


impl TsPoly{
    /// Highest default available power of the polynomial. Last element of coefs vector is coefitient next to x^(DEFAULT_MAX_POW-1).
    pub(crate) const DEFAULT_MAX_POW: usize = 30;

    pub fn new() -> TsPoly{
        TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: 0}
    }

    pub fn put_offset(&mut self, mut offset: f64){
        if offset == 0.0 {return;}
        offset = -offset;
        for power in 1..Self::DEFAULT_MAX_POW{
            if self.coefs[power] != 0.0 {
                let current_coef = self.coefs[power];
                self.coefs[0] += current_coef*offset.powf(power as f64);
                for index in 1..power{
                    self.coefs[power-index] += current_coef*Self::binomial_coef(power, index)*offset.powf(index as f64);
                }
            }
        }
    }

    pub fn from_const(constant: f64) -> TsPoly {
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: 0};
        temp.coefs[0] = constant;
        temp
    }

    /// Sets coefitiens to 0 from x^new_max_pow+1 monom till the end (DEFAULT_MAX_POW-1th power)
    pub fn truncate(&mut self, new_max_pow: usize){
        self.coefs[new_max_pow+1..Self::DEFAULT_MAX_POW].fill(0.0);
    }

    //TODO Make this function a table look up for speed
    pub(crate) fn binomial_coef(n: usize, k: usize) -> f64{
        if k > n {
            return 0.0;
        }
        if k == 0 || k == n {
            return 1.0;
        }
    
        let k = if k > n - k { n - k } else { k };
        let mut result = 1;
    
        for i in 0..k {
            result = result * (n - i) / (i + 1);
        }
    
        result as f64
    }

    //FIXME this operator doesn't work as intended, I don't get any offset improvement, the problem is with high powers of polynomials again
    pub fn of(&mut self, mut argument: TsPoly) {
        let mut per_power: Vec<TsPoly> = Vec::<TsPoly>::new();
        per_power.push(self.coefs[1]*argument.clone()); //argument^1
        for i in 2..=self.max_pow{
            argument*=argument.clone();
            if self.coefs[i] != 0.0 {
                per_power.push(self.coefs[i]*argument.clone()); //argument^i
            }
        }
        
        self.coefs[1..Self::DEFAULT_MAX_POW].fill(0.0);

        for poly in per_power {
            *self += poly;
        }
    }

    /// Returns the coefitient next to the highest power of x of the polynomial.
    pub(crate) fn lead(&self) -> f64{
        self.coefs[self.max_pow]
    }
}
