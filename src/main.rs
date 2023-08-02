// use std::{
//     env,
//     process::exit,
// };

use std::time::Instant;
use integral_aprox::*;


//potential bugs:
// - sin(x)^f(x), implement a power operation as a 3rd tier operation maybe

//features to implement:
// - checking weather the borders of integral are in the domains of a function
// - highlight the part of the function string that has typoes
// - remove all spaces and make all letters lowercase

// toy main:
fn main(){
    //let mut function = String::from("sin(x*7)*e^(x+1)-tg(x-8)/cos(x)");
    //let mut function = String::from("sin(x*7)*e^(x+1)+cos(x)*tg(x)");
    let mut function = String::from("3*x+7");

    let start = Instant::now();
    let tree = generate_tree_from_string(&mut function);
    let duration = start.elapsed();

    print_tree(&tree, 0, '\n');
    println!("Time spent: {:?}", duration);
    //measure_time!({});
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
    // let mut tree = Node::new();

    // generate_tree_from_string(&mut function, &mut tree);

        //generate assembly code from tree
        //generate machine code from that assembly and pack it in an executable
        //run executable and pass the result to this program
        //print aproxiation result with a funciton    
//     //print!("\n\n  {}\n\n ∫ 3*x + 7 dx  =  89.0\t\t(With {steps} steps)\n\n{}\n\n", end, start);
// }
