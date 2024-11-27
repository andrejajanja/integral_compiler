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
    pub(crate) coefs: Vec<f64>,
    pub(crate) max_pow: usize,
    pub(crate) from_x: bool
}

impl TsPoly{
    /// Highest default available power of the polynomial. Last element of coefs vector is coefitient next to x^(DEFAULT_MAX_POW-1).
    pub(crate) const DEFAULT_MAX_POW: usize = 30;

    pub fn zero() -> Self{
        Self { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: 0, from_x: true}
    }

    pub fn from_vec(mut provided_coefs: Vec<f64>, from_x: bool) -> Self{
        provided_coefs.resize(Self::DEFAULT_MAX_POW, 0.0);
        let mut temp_pow: usize = 0;
        for (i, coef) in provided_coefs.iter().enumerate().take(Self::DEFAULT_MAX_POW)  {
            if *coef != 0.0 {
                temp_pow = i;
            }
        }
        Self { coefs: provided_coefs, max_pow: temp_pow, from_x}
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

    pub fn from_const(constant: f64) -> Self {
        let mut temp = TsPoly { coefs: vec![0.0; Self::DEFAULT_MAX_POW], max_pow: 0, from_x: true};
        temp.coefs[0] = constant;
        temp
    }

    /// Sets coefitiens to 0 from x^new_max_pow+1 monom till the end (DEFAULT_MAX_POW-1th power)
    pub fn truncate(&mut self, new_max_pow: usize){
        self.max_pow = new_max_pow;
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

    pub fn generate_ir(&self, poly_argument: Option<String>, start_addr: u16) -> (String, String){
        let mut x: String = String::from("%x");
        if let Some(temp_argument) = poly_argument { x = temp_argument;}

        let mut temp = format!(
r"%p0_{} = fadd double 0.0, {:.15e}
%tmul0_{} = fmul double {:.15e}, {}
%p1_{} = fadd double %tmul0_{}, %p0_{}
%tpow1_{} = fmul double %x, %x
%tmul1_{} = fmul double {:.15e}, %tpow1_{}
%p2_{} = fadd double %tmul1_{}, %p1_{}
",
start_addr, self.coefs[0],
start_addr, self.coefs[1], x,
start_addr, start_addr, start_addr,
start_addr,
start_addr, self.coefs[1], start_addr,
start_addr, start_addr, start_addr
        );

        for i in 2..=self.max_pow-1 {
            temp += format!(
r"%tpow{}_{} = fmul double %tpow{}_{}, {}
%tmul{}_{} = fmul double {:.15e}, %tpow{}_{}
%p{}_{} = fadd double %tmul{}_{}, %p{}_{}
",
i, start_addr, i-1, start_addr, x,
i, start_addr, self.coefs[i+1], i, start_addr,
i+1, start_addr, i, start_addr, i, start_addr,
).as_str();
        }
        let virtual_register = format!("%p{}_{}", self.max_pow, start_addr);
        (temp, virtual_register)
    }

    pub fn generate_ir_from_existing_powers(&self, start_addr: u16, existing_pow_start_addr: u16) -> (String, String){
        let mut temp = format!(
r"%s0_{} = fadd double 0.0, {:.15e}
%tmul0_{} = fmul double {:.15e}, %tpow0_{}
%s1_{} = fadd double %tmul0_{}, %s0_{}
",
start_addr, self.coefs[0],
start_addr, self.coefs[1], existing_pow_start_addr,
start_addr, start_addr, start_addr,
        );

        for i in 1..self.max_pow {
            temp += format!(
r"%tmul{}_{} = fmul double {:.15e}, %tpow{}_{}
%s{}_{} = fadd double %tmul{}_{}, %s{}_{}
",
i, start_addr, self.coefs[i+1], i, existing_pow_start_addr,
i+1, start_addr, i, start_addr, i, start_addr,
).as_str();
        }
        let virtual_register = format!("%s{}_{}", self.max_pow, start_addr);
        (temp, virtual_register)
    }

}
