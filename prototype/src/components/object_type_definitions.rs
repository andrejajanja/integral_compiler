use crate::{components::terminal_decoration::Color, unrecoverable_error};
use std::{
    fmt,
    process::exit
};

//TODO Add hyperbolic functions to this enum
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Func {
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

    //Exp based functions
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
    Ob, //open bracket
    Cb, //closed bracket

    //auxilary
    X,      //function variable
    Const,  // C, C e R
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
            Func::Sinh => todo!(),
            Func::Cosh => todo!(),
            Func::Tgh => todo!(),
            Func::Ctgh => todo!(),
            Func::Arsinh => todo!(),
            Func::Arcosh => todo!(),
            Func::Artgh => todo!(),
            Func::Arctgh => todo!(),
            Func::Ob | Func::Cb | Func::None | Func::Const | Func::X => {
                unrecoverable_error!(
                    "Error generating the IR code string",
                    format!("'Func::{:?}' was encountered, which shouldn't be there.", self)
                );
            },
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
            Func::Const => String::from("Const"),
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
            Func::Arctgh => String::from("arctgh")
        };

        write!(f, "{}", temp)
    }
}

#[derive(Debug,Clone)]
pub struct Node {
    pub first: Option<Box<Node>>,
    pub second: Option<Box<Node>>,
    pub op: Func,
    pub c: Option<f64>, //if type op = Func::Const
}

impl Node {
    pub fn new() -> Node {
        Node {
            first: None,
            second: None,
            op: Func::None,
            c: None,
        }
    }

    pub fn new_value(fun: Func, con: Option<f64>) -> Node {
        Node {
            first: None,
            second: None,
            op: fun,
            c: con,
        }
    }
}
