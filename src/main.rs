mod parts;
mod stages;

// use std::process::exit;
// use std::env;
// use parts::auxilary_functions::{print_help, parse_inputs};
use crate::stages::string_to_ir::generate_ir;
use crate::parts::auxilary_functions::wrap_ir_code;
use std::fs;
use std::process::Command;
// use crate::stages::string_to_tree_iterative::str_to_tree_iter;
// use crate::stages::string_to_tree_recursive::print_tree_rec;

//toy main:
fn main(){
    let _a = 5;
    let _b = 10;
    let _steps = 1000000;
    //let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)");
    let function = String::from("sin(x)*e^(x)+cos(x)*ln(x)");
    //let function = String::from("sin(x)");
    //let function = String::from("3*x+7");
    //let function = String::from("sin(x)+7*e^(atg(x+11))");    
    
    let ir_code = wrap_ir_code(generate_ir(&function));
    fs::write("/home/andreja/Documents/rust_projects/integral_aprox/IR_code.ll", ir_code).expect("neka greska");

    let _kompilacija = Command::new("bash")
        .arg("/home/andreja/Documents/rust_projects/integral_aprox/compile_parts.sh")
        .output()
        .expect("Greska u kompilaciji");
    
    let output = Command::new("/home/andreja/Documents/rust_projects/integral_aprox/executable")
        .output()
        .expect("Greska u kompilaciji");

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}

// whole main, DO SOME WORK ON USER FUCKING EXPERIANCE
// fn main() {
//     let provided: Vec<String> = env::args().collect();
//     if provided.len() != 1 {
//         if provided[1] == "--help"{
//             print_help();
//             exit(0);
//         }
//     }
//     let (mut function, start, end, steps) = parse_inputs();
//     function = function.replace(" ", "");

//     let ir_code = generate_ir(&function);
//     println!("{}", ir_code);

//     // generate machine code from that assembly and pack it in an executable
//     // run executable and pass the result to this program
//     // print aproxiation result with a funciton    
//     print!("\n\n  {}\n\n ∫ 3*x + 7 dx  =  89.0\t\t(With {steps} steps)\n\n{}\n\n", end, start);
// }

#[cfg(test)]
mod tests {
    mod unit_tree;
}