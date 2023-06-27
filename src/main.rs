// use std::{
//     env,
//     process::exit,
// };
use integral_aprox::*;

//features to implement:
//checking weather the borders of integral are in the domains of a function


// toy main:
fn main(){
    //let mut function = String::from("sin(x*7)*e^(x+1)-tg(x-8)+cos(x)");
    //let mut function = String::from("sin(x*7)*e^(x+1)+cos(x)tg(x)");
    let mut function = String::from("x+cos(x)");

    let mut tree = Node::new();
    generate_tree_from_string(&mut function, &mut tree);

    println!("{:?}", tree.op);
}

// whole main
// fn main() {
//     let provided: Vec<String> = env::args().collect();
//     if provided.len() != 1 {
//         if provided[1] == "--help"{
//             print_help();
//             exit(0);
//         }
//     }

//     let mut function = String::new();
//     let mut start: f64;
//     let mut end: f64;
//     let mut steps: u64;

//     parse_inputs(&mut function, &mut start, &mut end, &mut steps);
//     function = function.replace(" ", "");

    
//     //print!("\n\n  {}\n\n ∫ 3*x + 7 dx  =  89.0\t\t(With {steps} steps)\n\n{}\n\n", end, start);
// }
