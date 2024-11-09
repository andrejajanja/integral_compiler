#![allow(dead_code)]
#![allow(unused_doc_comments)]
use crate::unrecoverable_error;
use std::process::exit;
use crate::components::{
    object_type_definitions::*,
    terminal_decoration::Color,
    polynomials::TsPoly
};

/// # States:
/// TODO Make this state explanation better
/// Lexem processor goes trough state while it processes all the lexems.
/// Here is a list of all valid, defined states that exist, that cover every combination of every function user can provide
/// 
/// 0. default;
/// 1. detected just x, waiting for something else;
/// 2. detected just const, waiting for something else;
/// 3. detected x and const, respectively, waiting for the operation between them;
/// 4. detected const and x, respectively, waiting for the operation between them.
pub struct LexemProcessorTaylor{
    pub(crate) lexems: Vec<Node>,
    pub(crate) gen_polys: Vec<TsPoly>,
    pub(crate) poly_ops: Vec<Func>,
    pub(crate) state: u8,
    pub(crate) max_power: usize,
    pub(crate) temp_const: f64,
    pub(crate) current_lexem: Node,
    pub(crate) precision_center: f64
}

impl LexemProcessorTaylor{
    //TODO Add some custom enum to determine ops and that stuff

    pub fn new(lexems: Vec<Node>, precision_center: f64, max_power: usize) -> LexemProcessorTaylor {
        LexemProcessorTaylor{
            lexems: lexems,
            gen_polys: Vec::<TsPoly>::new(),
            poly_ops: Vec::<Func>::new(),
            state: 0,
            max_power: max_power,
            temp_const: 0.0,
            current_lexem: Node::new(),
            precision_center: precision_center,
        }
    }

    pub fn process_lexems(&mut self) {
        for elem in self.lexems.clone(){
            self.current_lexem = elem.clone();
            println!("{} - {}", self.current_lexem.op, self.state);

            if elem.op == Func::Const{
                self.temp_const = match elem.c {
                    Some(c) => c,
                    None => {
                        unrecoverable_error!(
                            "Lexem processor error | Invalid postfix node",
                            "Node with Const op has None for c value"
                        );
                    },
                };

                if self.state == 1{
                    self.state = 3;
                }else{
                    self.state = 2;
                }

                continue;
            }

            if elem.op == Func::X {
                if self.state == 0 {
                    self.state = 1;
                }else{
                    self.state = 4;
                }
                continue;
            }

            match self.state {
                //State 0 handles multiple things at once, think about making this separate states, most notably poly to poly ops
                0 => self.state_0_handler(),
                1 => self.state_1_handler(),
                2 => self.state_2_handler(),
                3 => self.state_3_handler(),
                4 => self.state_4_handler(),
                unsupported_state => {
                    unrecoverable_error!(
                        "Lexer processor error | Encountered unknown state",
                        unsupported_state
                    );
                }
            }
        }
    }

    pub fn generate_ir_code(&mut self) -> (String, i16){
        (String::from("aa"), 0)
    }
}