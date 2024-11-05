use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use crate::components::object_type_definitions::Func;
use std::{
    fmt,
    process::exit,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign}
};

//TODO write description for everything defined for this struct
#[derive(Debug, Clone)]
pub struct TsPoly {
    pub coefs: Vec<f64>,
    pub max_pow: usize
}

impl TsPoly{
    pub(crate) const DEFAULT_MAX_POW: usize = 30;

    pub fn from_func(fun: Func, mut offset: f64, max_p: usize) -> TsPoly{
        if max_p >= Self::DEFAULT_MAX_POW {
            unrecoverable_error!(
                "Frontend error | Invalid argument max_p when generating Taylor's polynomial for a Func value",
                format!("max_p({}) >= DEFAULT_MAX_POW({})", max_p, Self::DEFAULT_MAX_POW-1)
            );
        }

        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p};
        
        match fun{
            Func::Sin => Self::generate_sin(&mut temp, &mut offset, max_p),
            Func::Cos => Self::generate_cos(&mut temp, &mut offset, max_p),
            Func::Tg => Self::generate_tg(&mut temp, &mut offset, max_p),
            Func::Ctg => todo!(),
            Func::Ln => Self::generate_ln(&mut temp, offset, max_p),
            Func::Exp => Self::generate_exp(&mut temp, offset, max_p),
            Func::Atg => todo!(),
            Func::Actg => todo!(),
            Func::Asin => todo!(),
            Func::Acos => todo!(),
            _ => {
                unrecoverable_error!("Frontend error | Can't/Shouldn't generate Taylor's polynomial for this Func value", fun);
            }
        }

        if offset == 0.0{
            return temp;
        }

        offset = -offset;

        for power in 1..Self::DEFAULT_MAX_POW{
            if temp.coefs[power] != 0.0 {
                let current_coef = temp.coefs[power];
                temp.coefs[0] += current_coef*offset.powf(power as f64);
                for index in 1..power{
                    temp.coefs[power-index] += current_coef*Self::binomial_coef(power, index)*offset.powf(index as f64);
                }
            }
        }

        temp
    }

    pub fn truncate(&mut self, max: usize){
        for index in max+1..Self::DEFAULT_MAX_POW{
            self.coefs[index] = 0.0;
        }
    }

    fn binomial_coef(n: usize, k: usize) -> f64{
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
}

//Definitions of overloaded traits for ergonomic access

impl Add for TsPoly{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_p;

        if self.max_pow > rhs.max_pow{
            max_p = self.max_pow;
        }else{
            max_p = rhs.max_pow
        }

        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p};

        for i in 0..Self::DEFAULT_MAX_POW{
            temp.coefs[i] = self.coefs[i] + rhs.coefs[i];
        }

        temp
    }
}

impl AddAssign for TsPoly{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..Self::DEFAULT_MAX_POW{
            self.coefs[i]+=rhs.coefs[i];
        }
    }
}

impl Sub for TsPoly{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        //TODO check integrity of this max_pow code
        let max_p;

        if self.max_pow > rhs.max_pow{
            max_p = self.max_pow;
        }else{
            max_p = rhs.max_pow
        }

        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: max_p};

        for i in 0..Self::DEFAULT_MAX_POW{
            temp.coefs[i] = self.coefs[i] - rhs.coefs[i];
        }

        temp
    }
}

impl SubAssign for TsPoly{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..Self::DEFAULT_MAX_POW{
            self.coefs[i]-=rhs.coefs[i];
        }
    }
}

impl Mul for TsPoly{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output{
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow + rhs.max_pow};

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
        for i_lhs in 0..Self::DEFAULT_MAX_POW{
            for i_rhs in 0..Self::DEFAULT_MAX_POW{
                let end_index = i_lhs + i_rhs;
                if end_index >= Self::DEFAULT_MAX_POW { 
                    break; 
                }
                self.coefs[end_index] += self.coefs[i_lhs] * rhs.coefs[i_rhs];
            }   
        }
    }
}

impl Div for TsPoly{
    type Output = Self;

    //TODO Check the integrity of this operator
    fn div(self, rhs: Self) -> Self::Output {
        let mut remainder = self.clone();
        let mut quotient = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow - rhs.max_pow};

        // Perform division algorithm
        for i in 0..=self.max_pow - rhs.max_pow {
            quotient.coefs[i] = remainder.coefs[i] / rhs.coefs[0];

            for j in 0..=rhs.max_pow {
                remainder.coefs[i + j] -= quotient.coefs[i] * rhs.coefs[j];
            }
        }

        quotient
    }
}

impl DivAssign for TsPoly{
    fn div_assign(&mut self, rhs: Self) {
        if rhs.max_pow > self.max_pow {
            unrecoverable_error!(
                "Frontend error | Power of the numerator needs to be bigger than the power of denuminator polynomial (negative powers are not supported)",
                format!("Numerator/Left operand({}) < Denuminator/Right operand({})", self.max_pow, rhs.max_pow)
            );
        }
        

        let mut quotient = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: self.max_pow - rhs.max_pow};

        //TODO optimize this algorithm
        for i in 0..=self.max_pow - rhs.max_pow {
            quotient.coefs[i] = self.coefs[i] / rhs.coefs[0];

            for j in 0..=rhs.max_pow {
                self.coefs[i + j] -= quotient.coefs[i] * rhs.coefs[j];
            }

            println!("{} // {}", self, quotient);
        }

        self.max_pow-=rhs.max_pow;
        for i in 0..Self::DEFAULT_MAX_POW{
            self.coefs[i] = quotient.coefs[i];
        }
    }
}

impl fmt::Display for TsPoly{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp_str = String::from("");

        let mut started = false;

        for index in (0..Self::DEFAULT_MAX_POW).rev(){
            if self.coefs[index] == 0.0 {
                continue;
            }

            if started && self.coefs[index] > 0.0{
                temp_str += " + ";
            }

            if self.coefs[index] != 1.0 {
                temp_str += &self.coefs[index].to_string();
            }
            match index {
                0 => {
                    if self.coefs[index] == 1.0 {
                        temp_str += "1";
                    }
                },
                1 => {
                    if self.coefs[index] != 1.0{
                        temp_str += "*";
                    }
                    temp_str += "x";
                },
                _ => {
                    if self.coefs[index] != 1.0{
                        temp_str += "*";
                    }

                    temp_str += "x^";
                    temp_str += &index.to_string();
                }
            }

            started = true;
        }

        if !started {
            temp_str += "0";
        }

        write!(f, "{}", temp_str)
    }
}

