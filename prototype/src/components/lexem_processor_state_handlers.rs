use crate::unrecoverable_error;

use super::{
    lexem_processor_taylor::{LexemProcessorTaylor, Operand, Relation},
    object_type_definitions::Func,
    terminal_decoration::Color,
    polynomials::TsPoly
};

use std::{f64::consts::PI, process::exit};

impl LexemProcessorTaylor{

    //TODO check integrity of every branch in this function
    pub(crate) fn state_0_handler(&mut self){

        let temp_operand = match self.operands.pop() {
            Some(_value) => _value,
            None => {
                unrecoverable_error!(
                    "Lexem processor error | State 0 expected at least operand in operands vector",
                    "found none"
                );
            }
        };

        match self.current_lexem.op {
            //Handling polynomials upon which I can easaly do 'of' operation
            Func::Sin | Func::Cos | Func::Ln | Func::Exp | Func::Sinh | Func::Cosh => {
                let mut temp_poly = TsPoly::from_func(self.current_lexem.op, self.precision_center, self.max_power);
                match temp_operand {
                    Operand::Polynomial(ts_poly) => {
                        temp_poly.of(ts_poly.to_owned());
                    },
                    some_other_operand => {
                        self.operands.push(some_other_operand);
                        self.relations.push(Relation::Of);
                    }
                }    
                self.operands.push(Operand::Polynomial(temp_poly));    
            },
            Func::Sqrt => {
                self.operands.push(temp_operand);
                self.relations.push(Relation::Sqrt);
            },
            Func::Tg => todo!(),
            Func::Ctg => todo!(),
            Func::Tgh => todo!(),
            Func::Ctgh => todo!(),
            Func::Atg => todo!(),
            Func::Actg => todo!(),
            Func::Asin => todo!(),
            Func::Acos => todo!(),
            Func::Arsinh => todo!(),
            Func::Arcosh => todo!(),
            Func::Artgh => todo!(),
            Func::Arctgh => todo!(),
            Func::Add => todo!(),
            Func::Sub => todo!(),
            Func::Mul => todo!(),
            Func::Div => todo!(),
            Func::Pow => todo!(),
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Invlid lexem for state 0",
                    format!("{unsupported_op}")
                );
            }
        }
    }

    pub(crate) fn state_1_handler(&mut self){
        match self.current_lexem.op {
            Func::Sin => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Sin, self.precision_center, self.max_power))),
            Func::Cos => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Cos, self.precision_center, self.max_power))),
            Func::Tg => todo!("Implement Taylor generation for Tg"),
            Func::Ctg => todo!("Implement Taylor generation for Ctg"),
            Func::Ln => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Ln, self.precision_center, self.max_power))),
            Func::Exp => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Exp, self.precision_center, self.max_power))),
            Func::Sinh => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Sinh, self.precision_center, self.max_power))),
            Func::Cosh => self.operands.push(Operand::Polynomial(TsPoly::from_func(Func::Cosh, self.precision_center, self.max_power))),
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
            Func::Sqrt => self.operands.push(Operand::SqrtOfX),
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Can't generate Taylor sequence for this combination of operands",
                    format!("{unsupported_op}(x)")
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
            Func::Sqrt => self.temp_const = self.temp_const.sqrt(),
            Func::Add => todo!("Handle binary ops for state 2 - add"), //FIXME All binary ops should put state to 0
            Func::Sub => todo!("Handle binary ops for state 2 - sub"),
            Func::Mul => todo!("Handle binary ops for state 2 - mul"),
            Func::Div => todo!("Handle binary ops for state 2 - div"),
            Func::Pow => todo!("Handle binary ops for state 2 - pow"),
            unsupported_op => {
                unrecoverable_error!(
                    "Lexem processor error | Unsupported operation on a constant in a state 2 (see docs for more details)",
                    format!("{}", unsupported_op)
                );
            }
        }
        self.state = 2; //Need this in order to have chained interpreter calls on a const value, if it's not binry op ofc
    }

    pub(crate) fn state_3_handler(&mut self){
        match self.current_lexem.op {
            Func::Add => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = -self.temp_const;
                temp_poly.coefs[1] = 1.0;
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
            },
            Func::Div => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = 1.0/self.temp_const; //FIXME Handle if temp_const is 0 here, to stop the entire thing
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
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
                    self.operands.push(Operand::Polynomial(temp_poly));
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
                self.operands.push(Operand::Polynomial(temp_poly));
            },
            Func::Sub => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.coefs[1] = -1.0;
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
            },
            Func::Mul => {
                let mut temp_poly = TsPoly::new();
                temp_poly.coefs[1] = self.temp_const;
                temp_poly.max_pow = 1;
                self.operands.push(Operand::Polynomial(temp_poly));
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