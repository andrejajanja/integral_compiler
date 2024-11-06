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
            Func::Tg => todo!(),
            Func::Ctg => todo!(),
            Func::Ln => Self::generate_ln(&mut temp, offset, max_p),
            Func::Exp => Self::generate_exp(&mut temp, offset, max_p),
            Func::Atg => todo!(),
            Func::Actg => todo!(),
            Func::Asin => todo!(),
            Func::Acos => todo!(),
            Func::Sinh => Self::generate_sinh(&mut temp, offset, max_p),
            Func::Cosh => Self::generate_cosh(&mut temp, offset, max_p),
            Func::Tgh => todo!(),
            Func::Ctgh => todo!(),
            Func::Arsinh => todo!(),
            Func::Arcosh => todo!(),
            Func::Artgh => todo!(),
            Func::Arctgh => todo!(),
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

    pub fn truncate(&mut self, new_max_pow: usize){
        self.coefs[new_max_pow+1..Self::DEFAULT_MAX_POW].fill(0.0);
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

    pub(crate) fn lead(&self) -> f64{
        self.coefs[self.max_pow]
    }
}
