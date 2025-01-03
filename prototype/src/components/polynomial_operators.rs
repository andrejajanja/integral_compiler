#![allow(unused_imports)]
use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;

use super::polynomials::TsPoly;
use std::{
    fmt,
    process::exit,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign}
};

impl Add for TsPoly{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let temp_pow = if self.max_pow >= rhs.max_pow { self.max_pow } else { rhs.max_pow };

        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: temp_pow, from_x: true};
        for i in 0..=temp_pow {
            temp.coefs[i] = self.coefs[i] + rhs.coefs[i];
        }

        temp
    }
}

impl AddAssign for TsPoly{
    fn add_assign(&mut self, rhs: Self) {
        let temp_pow = if self.max_pow >= rhs.max_pow { self.max_pow } else { rhs.max_pow };
        for i in 0..=temp_pow {
            self.coefs[i]+=rhs.coefs[i];
        }
        self.max_pow = temp_pow;
    }
}

impl Sub for TsPoly{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let temp_pow = if self.max_pow >= rhs.max_pow { self.max_pow } else { rhs.max_pow };

        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: temp_pow, from_x: true};
        for i in 0..=temp_pow{
            temp.coefs[i] = self.coefs[i] - rhs.coefs[i];
        }

        temp
    }
}

impl SubAssign for TsPoly{
    fn sub_assign(&mut self, rhs: Self) {
        let temp_pow = if self.max_pow >= rhs.max_pow { self.max_pow } else { rhs.max_pow };
        for i in 0..=temp_pow {
            self.coefs[i]-=rhs.coefs[i];
        }
        self.max_pow = temp_pow;
    }
}

impl Mul for TsPoly{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output{
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow + rhs.max_pow, from_x: true};
        for i_lhs in 0..Self::DEFAULT_MAX_POW{
            for i_rhs in 0..Self::DEFAULT_MAX_POW{
                let end_index = i_lhs + i_rhs;
                if end_index > Self::DEFAULT_MAX_POW - 1 { 
                    break; 
                }
                temp.coefs[end_index] += self.coefs[i_lhs] * rhs.coefs[i_rhs];
            }   
        }

        temp
    }
}

impl MulAssign for TsPoly{
    fn mul_assign(&mut self, rhs: Self) {
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow + rhs.max_pow, from_x: true};
        for i_lhs in 0..Self::DEFAULT_MAX_POW{
            for i_rhs in 0..Self::DEFAULT_MAX_POW{
                let end_index = i_lhs + i_rhs;
                if end_index >= Self::DEFAULT_MAX_POW { 
                    break; 
                }
                temp.coefs[end_index] += self.coefs[i_lhs] * rhs.coefs[i_rhs];
            }   
        }

        let mut temp_pow = self.max_pow + rhs.max_pow;
        if temp_pow >= Self::DEFAULT_MAX_POW {
            temp_pow = Self::DEFAULT_MAX_POW - 1;
        }

        self.max_pow = temp_pow;
        for i in 0..=self.max_pow { self.coefs[i] = temp.coefs[i]; }
    }
}

//Next three operator overloads implement multiplying all coefs of a polynomial with a real number
impl Mul<f64> for TsPoly {
    type Output = TsPoly;

    fn mul(mut self, rhs: f64) -> TsPoly {
        for i in 0..=self.max_pow{
            self.coefs[i] *= rhs;
        }

        self
    }
}

impl Mul<TsPoly> for f64 {
    type Output = TsPoly;

    fn mul(self, mut rhs: TsPoly) -> TsPoly {
        for i in 0..=rhs.max_pow{
            rhs.coefs[i] *= self;
        }
        rhs
    }
}

impl MulAssign<f64> for TsPoly{
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..=self.max_pow{
            self.coefs[i] *= rhs;
        }
    }
}

// function n / d is
//     require d ≠ 0
//     q ← 0
//     r ← n             // At each step n = d × q + r
//
//     while r ≠ 0 and degree(r) ≥ degree(d) do
//         t ← lead(r) / lead(d)       // Divide the leading terms
//         q ← q + t
//         r ← r − t × d
// 
//     return (q, r) // here +,-,x are polynomial arithmetic operations 
impl Div for TsPoly{
    type Output = Self;       
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.max_pow == 0 && rhs.coefs[0] == 0.0 {
            unrecoverable_error!(
                "Taylor generation error | Polynomial division error",
                "Right hand side can't be 0-polynomial"
            );
        }
        
        let mut quotient = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow-rhs.max_pow, from_x: true};
        let mut remainder = self.clone();

        while remainder.coefs[0] != 0.0 && remainder.max_pow != 0 && remainder.max_pow >= rhs.max_pow {
            let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: remainder.max_pow-rhs.max_pow, from_x: true};
            temp.coefs[remainder.max_pow-rhs.max_pow] = remainder.lead()/rhs.lead();
            quotient+=temp.clone();
            remainder-=temp*rhs.clone();
            remainder.max_pow-=1;
        }

        quotient
    }
}

impl DivAssign for TsPoly{
    fn div_assign(&mut self, rhs: Self) {
        if rhs.max_pow == 0 && rhs.coefs[0] == 0.0 {
            unrecoverable_error!(
                "Taylor generation error | Polynomial division error",
                "Right hand side can't be 0-polynomial"
            );
        }
        
        let mut quotient = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow-rhs.max_pow, from_x: true};

        while self.coefs[0] != 0.0 && self.max_pow != 0 && self.max_pow >= rhs.max_pow {
            let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow-rhs.max_pow, from_x: true};
            temp.coefs[self.max_pow-rhs.max_pow] = self.lead()/rhs.lead();
            quotient+=temp.clone();
            *self-=temp*rhs.clone();
            self.max_pow-=1;
        }

        for i in 0..Self::DEFAULT_MAX_POW{
            self.coefs[i] = quotient.coefs[i];
        }
    }
}

impl fmt::Display for TsPoly{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp_str = String::new();
        let mut started = false;

        for index in (0..=self.max_pow).rev(){
            if self.coefs[index] == 0.0 { continue; }

            if started && self.coefs[index] > 0.0 { temp_str += "+ "; }

            if self.coefs[index] != 1.0 { temp_str += &format!("{:.10}", &self.coefs[index]); }

            match index {
                0 => {},
                1 => {
                    if self.coefs[index] != 1.0{
                        temp_str += "*";
                    }
                    temp_str += "x ";
                },
                _ => {
                    if self.coefs[index] != 1.0{
                        temp_str += "*";
                    }

                    temp_str += &format!("x^{} ", index);
                }
            }

            started = true;
        }

        if !started {temp_str += "0";}

        temp_str += match self.from_x {
            true => " X",
            false => " R",
        };

        write!(f, "{}", temp_str)
    }
}
