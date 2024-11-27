#![allow(unused_imports)]
use crate::{components::terminal_decoration::Color, unrecoverable_error};
use std::{
    fmt,
    process::exit
};

use super::polynomials::TsPoly;

//TODO Add hyperbolic functions to this enum
#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    //Values used in subsequent optimization passes
    Poly(TsPoly),

    //Trigonometry functions
    Sin,    // sin(f(x))
    Cos,    // cos(f(x))
    Tg,     // tg(f(x))
    Ctg,    // ctg(f(x))
    Sinh,   // sinh(f(x))
    Cosh,   // cosh(f(x))
    Tgh,    // tgh(f(x))
    Ctgh,   // ctgh(f(x))

    //Inverse trigonometry functions
    Atg,  // arctg(f(x))
    Actg, // arcctg(f(x))
    Asin, // arcsin(f(x))
    Acos, // arccos(f(x))
    Arsinh,   // arsinh(f(x))
    Arcosh,   // arcosh(f(x))
    Artgh,   // artgh(f(x))
    Arctgh,   // arctgh(f(x))

    //Exp functions
    Ln,     // ln(f(x))
    Exp,    // e^(f(x))

    //Algebraic functions/
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,   // sqrt(f(x))
    Pow,    // g(x)^(f(x))

    //brackets
    Ob, //open
    Cb, //closed

    //auxilary
    X,      //function variable
    Const(f64),  // C, C e R
    None,   // end of the tree
}


impl Func {
    pub fn ir_string(&self) -> String {
        match self{
            Func::Add => String::from("fadd"),
            Func::Sub => String::from("fsub"),
            Func::Mul => String::from("fmul"),
            Func::Div => String::from("fdiv"),
            Func::Pow => String::from("pow"),
            Func::Sin => String::from("sin"),
            Func::Cos => String::from("cos"),
            Func::Tg | Func::Ctg => String::from("tan"),
            Func::Ln => String::from("ln"),
            Func::Exp => String::from("exp"),
            Func::Sqrt => String::from("sqrt"),
            Func::Atg | Func::Actg => String::from("atan"),
            Func::Asin => String::from("asin"),
            Func::Acos => String::from("acos"),
            Func::Sinh => todo!("sinh"),
            Func::Cosh => todo!("cosh"),
            Func::Tgh => todo!("tgh"),
            Func::Ctgh => todo!("ctgh"),
            Func::Arsinh => todo!("arsinh"),
            Func::Arcosh => todo!("arcosh"),
            Func::Artgh => todo!("artgh"),
            Func::Arctgh => todo!("arctgh"),
            Func::Ob | Func::Cb | Func::None | Func::Const(_) | Func::X => {
                unrecoverable_error!(
                    "Error generating the IR code string",
                    format!("'Func::{:?}' was encountered, which shouldn't be there.", self)
                );
            },
            Func::Poly(_ts_poly) => todo!(),
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp: String = match self{
            Func::Add => String::from("+"),
            Func::Sub => String::from("-"),
            Func::Mul => String::from("*"),
            Func::Div => String::from("/"),
            Func::Pow => String::from("^"),
            Func::X => String::from("x"),
            Func::None => String::from("None"),
            Func::Sin => String::from("sin"),
            Func::Cos => String::from("cos"),
            Func::Tg => String::from("tg"),
            Func::Ctg => String::from("ctg"),
            Func::Ln => String::from("ln"),
            Func::Exp => String::from("e^"),
            Func::Sqrt => String::from("sqrt"),
            Func::Const(value) => value.to_string(),
            Func::Atg => String::from("arctg"),
            Func::Asin => String::from("arcsin"),
            Func::Acos => String::from("arccos"),
            Func::Actg => String::from("arcctg"),
            Func::Ob => String::from("("),
            Func::Cb => String::from(")"),
            Func::Sinh => String::from("sinh"),
            Func::Cosh => String::from("cosh"),
            Func::Tgh => String::from("tgh"),
            Func::Ctgh => String::from("ctgh"),
            Func::Arsinh => String::from("arsinh"),
            Func::Arcosh => String::from("arcosh"),
            Func::Artgh => String::from("artgh"),
            Func::Arctgh => String::from("arctgh"),
            Func::Poly(ts_poly) => ts_poly.to_string(),
        };

        write!(f, "{}", temp)
    }
}

#[derive(Debug,Clone)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub op: Func,
}

impl Node {
    pub fn new() -> Node {
        Node {
            left: None,
            right: None,
            op: Func::None,
        }
    }

    pub fn from_value(value: f64) -> Node {
        Node {
            left: None,
            right: None,
            op: Func::Const(value)
        }
    }

    pub fn from_func(operation: Func) -> Node {
        Node {
            left: None,
            right: None,
            op: operation
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
