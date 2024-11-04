use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use crate::components::object_type_definitions::Func;
use std::{
    fmt,
    process::exit,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign}
};

//TODO write description for everything defined for this struct
#[derive(Debug, Clone)]
pub struct TsPoly {
    pub coefs: Vec<f64>
}

impl TsPoly{
    pub(crate) const DEFAULT_MAX_POW: usize = 30;

    pub fn test(&mut self, mut offset: f64) {
        offset = -offset;

        //FIXME offseting the polynomial doesn't garantee the percision gains, here is the error, probably
        for power in 1..Self::DEFAULT_MAX_POW{
            if self.coefs[power] != 0.0 {
                let current_coef = self.coefs[power];
                self.coefs[0] += current_coef*offset.powi(power as i32);
                for index in 1..power{
                    self.coefs[power-index] += current_coef*Self::binomial_coef(power, index)*offset.powi(index as i32);
                }
            }
        }
    }

    pub fn from_func(fun: Func, mut offset: f64, max_p: usize) -> TsPoly{
        if max_p >= Self::DEFAULT_MAX_POW {
            unrecoverable_error!(
                "Frontend error | Invalid argument max_p when generating Taylor's polynomial",
                format!("max_p({}) >= DEFAULT_MAX_POW({})", max_p, Self::DEFAULT_MAX_POW-1)
            );
        }

        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW]};
        
        match fun{
            Func::Sin => Self::generate_sin(&mut temp, &mut offset, max_p),
            Func::Cos => Self::generate_cos(&mut temp, &mut offset, max_p),
            Func::Tg => todo!(),
            Func::Ctg => todo!(),
            Func::Ln => todo!(),//Self::generate_ln(&mut temp, offset),
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
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW]};

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
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW]};

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
        let mut temp = TsPoly{coefs: vec![0.0; Self::DEFAULT_MAX_POW]};

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
