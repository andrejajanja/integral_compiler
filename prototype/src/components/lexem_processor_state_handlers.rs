use crate::unrecoverable_error;

use super::{
    lexem_processor_taylor::LexemProcessorTaylor,
    object_type_definitions::Func,
    terminal_decoration::Color,
    polynomials::TsPoly
};

use std::{f64::consts::PI, process::exit};

impl LexemProcessorTaylor{
    pub(crate) fn state_0_handler(&mut self){

    }

    pub(crate) fn state_1_handler(&mut self){
        match self.current_lexem.op {
            Func::Sin => self.gen_polys.push(TsPoly::from_func(Func::Sin, self.precision_center, self.max_power)),
            Func::Cos => self.gen_polys.push(TsPoly::from_func(Func::Cos, self.precision_center, self.max_power)),
            Func::Tg => todo!("Implement Taylor generation for Tg"),
            Func::Ctg => todo!("Implement Taylor generation for Ctg"),
            Func::Ln => self.gen_polys.push(TsPoly::from_func(Func::Ln, self.precision_center, self.max_power)),
            Func::Exp => self.gen_polys.push(TsPoly::from_func(Func::Exp, self.precision_center, self.max_power)),
            Func::Sinh => todo!("Implement Taylor generation for Sinh"),
            Func::Cosh => todo!("Implement Taylor generation for Cosh"),
            Func::Tgh => todo!("Implement Taylor generation for Tgh"),
            Func::Ctgh => todo!("Implement Taylor generation for Ctgh"),
            Func::Atg => todo!("Implement Taylor generation for Atg"),
            Func::Actg => todo!("Implement Taylor generation for Actg"),
            Func::Asin => todo!("Implement Taylor generation for Asin"),
            Func::Acos => todo!("Implement Taylor generation for Acos"),
            Func::Arsinh => todo!("Implement Taylor generation for Arsinh"),
            Func::Arcosh => todo!("Implement Taylor generation for Arcosh"),
            Func::Artgh => todo!("Implement Taylor generation for Artgh"),
            Func::Arctgh => todo!("Implement Taylor generation for Arctgh"),
            Func::Sqrt => todo!("Implement sqrt(x)"),
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Can't generate Taylor sequence for this combination of operands",
                    format!("{}(x)", unsupported_op)
                );
            }
        }

        self.state = 0;
    }

    pub(crate) fn state_2_handler(&mut self){
        match self.current_lexem.op {
            Func::Sin => self.temp_const = self.temp_const.sin(),
            Func::Cos => self.temp_const = self.temp_const.cos(),
            Func::Tg => self.temp_const = self.temp_const.tan(),
            Func::Ctg => self.temp_const = PI/2.0 - self.temp_const.tan(),
            Func::Sinh => self.temp_const = self.temp_const.sinh(),
            Func::Cosh => self.temp_const = self.temp_const.cosh(),
            Func::Tgh => self.temp_const = self.temp_const.tanh(),
            Func::Ctgh => todo!("How do I calculate ctanh"),
            Func::Atg => self.temp_const = self.temp_const.atan(),
            Func::Actg => todo!("How do I calculate actan"),
            Func::Asin => self.temp_const = self.temp_const.asin(),
            Func::Acos => self.temp_const = self.temp_const.acos(),
            Func::Arsinh => self.temp_const = self.temp_const.asinh(),
            Func::Arcosh => self.temp_const = self.temp_const.acosh(),
            Func::Artgh => self.temp_const = self.temp_const.atanh(),
            Func::Arctgh => todo!("How do I calculate actanh"),
            Func::Ln => self.temp_const = self.temp_const.ln(),
            Func::Exp => self.temp_const = self.temp_const.exp(),
            Func::Add => todo!(),
            Func::Sub => todo!(),
            Func::Mul => todo!(),
            Func::Div => todo!(),
            Func::Sqrt => self.temp_const = self.temp_const.sqrt(),
            Func::Pow => todo!(),
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Unsupported operation on a constant in a state 2 (see docs for more details)",
                    format!("{}", unsupported_op)
                );
            }
        }
        self.state = 0;
    }

    pub(crate) fn state_3_handler(&mut self){
        match self.current_lexem.op {
            Func::Add => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = -self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Div => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = 1.0/self.temp_const; //FIXME Handle if temp_const is 0 here, to stop the entire thing
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Pow => {
                if self.temp_const.fract() == 0.0 {
                    let temp_pow = self.temp_const as usize;
                    if temp_pow >= TsPoly::DEFAULT_MAX_POW {
                        unrecoverable_error!(
                            "Lexem processor error | Power of x is too high to be converted to polynomial",
                            format!("Provided power: {}, max supported power {}",temp_pow, TsPoly::DEFAULT_MAX_POW)
                        );
                    }

                    let mut temp_poly = TsPoly::new();
                    temp_poly.max_pow = temp_pow;
                    temp_poly.coefs[temp_pow] = 1.0;
                    self.gen_polys.push(temp_poly);
                }else{
                    todo!("Compiler should add powf from std here to achieve: x^{}", self.temp_const);
                }
            },
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Unsupported operation between state 3 operands (see docs for more details)",
                    format!("{}", unsupported_op)
                );
            }
        }

        self.state = 0;
    }

    pub(crate) fn state_4_handler(&mut self){
        match self.current_lexem.op {
            Func::Add => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = -1.0;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.gen_polys.push(temp_poly);
            },
            Func::Div => todo!("For state 4, implement const/x combination"), //TOOD const/x
            Func::Pow => todo!("For state 4, implement const^x combination"), //TODO const^x
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Unsupported operation between state 4 operands (see docs for more details)",
                    format!("{}", unsupported_op)
                );
            }
        }

        self.state = 0;
    }

}