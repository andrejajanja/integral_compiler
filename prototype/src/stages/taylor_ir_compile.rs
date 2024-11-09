#![allow(dead_code)]
#![allow(unused_doc_comments)]
use crate::components::lexem_processor_taylor::LexemProcessorTaylor;

use crate::stages::function_parse_iterative::{parse_function, convert_infix_to_postfix};

pub fn generate_taylor_ir(function: &String, precision_center: f64, poly_degre: usize) -> String {
    let function_infix = parse_function(function);
    let function_postfix = convert_infix_to_postfix(function_infix);

    let mut lex_processor = LexemProcessorTaylor::new(function_postfix, precision_center, poly_degre);
    lex_processor.process_lexems();
    let (func_code, ret_addr) = lex_processor.generate_ir_code();

    format!("\ndefine double @fja(double %x){{\n{}\tret double %{}\n}}", func_code, ret_addr+1)
}
