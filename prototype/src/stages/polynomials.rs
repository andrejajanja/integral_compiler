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
    pub coefs: Vec<f64>,
    pub offset: f64
}

impl TsPoly{
    pub(crate) const DEFAULT_POW: usize = 30;

    pub fn from_func(fun: Func, offset: f64) -> TsPoly{
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
}

//Definitions of overloaded traits for ergonomic access

impl Add for TsPoly{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't add two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        let mut temp = TsPoly{coefs: vec![0.0; TsPoly::DEFAULT_POW], offset: self.offset};

        for i in 0..TsPoly::DEFAULT_POW{
            temp.coefs[i] = self.coefs[i] + rhs.coefs[i];
        }

        temp
    }
}

impl AddAssign for TsPoly{
    fn add_assign(&mut self, rhs: Self) {
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't add two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        for i in 0..TsPoly::DEFAULT_POW{
            self.coefs[i]+=rhs.coefs[i];
        }
    }
}

impl Sub for TsPoly{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't subtract two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        let mut temp = TsPoly{coefs: vec![0.0; TsPoly::DEFAULT_POW], offset: self.offset};

        for i in 0..TsPoly::DEFAULT_POW{
            temp.coefs[i] = self.coefs[i] - rhs.coefs[i];
        }

        temp
    }
}

impl SubAssign for TsPoly{
    fn sub_assign(&mut self, rhs: Self) {
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't subtract two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        for i in 0..TsPoly::DEFAULT_POW{
            self.coefs[i]-=rhs.coefs[i];
        }
    }
}

impl Mul for TsPoly{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output{
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't multiply two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        let mut temp = TsPoly{coefs: vec![0.0; TsPoly::DEFAULT_POW], offset: self.offset};

        for i_lhs in 0..TsPoly::DEFAULT_POW{
            for i_rhs in 0..TsPoly::DEFAULT_POW{
                let end_index = i_lhs + i_rhs;
                if end_index > TsPoly::DEFAULT_POW - 1 { 
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
        if self.offset != rhs.offset {
            unrecoverable_error!("Frontend error | Can't multiply two TsPolys with different offsets", format!("Left's offset: {}, Right's offset: {}", self.offset, rhs.offset));
        }

        for i_lhs in 0..TsPoly::DEFAULT_POW{
            for i_rhs in 0..TsPoly::DEFAULT_POW{
                let end_index = i_lhs + i_rhs;
                if end_index >= TsPoly::DEFAULT_POW { 
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

        let monome;
        if self.offset == 0.0 {
            monome = String::from("x");
        }else{
            monome = format!("(x-{})", self.offset);
        }

        for index in (0..TsPoly::DEFAULT_POW).rev(){
            if self.coefs[index] == 0.0 {
                continue;
            }

            if started && self.coefs[index] > 0.0{
                temp_str += "+";
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
                    temp_str += &monome;
                },
                _ => {
                    if self.coefs[index] != 1.0{
                        temp_str += "*";
                    }
                    temp_str += &monome;
                    temp_str += "^";
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
