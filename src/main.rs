mod parts;
mod stages;
//use std::time::Instant;

use parts::object_type_definitions::*;
use stages::string_to_tree_iterative::*;
//use stages::string_to_tree_recursive::*;

// toy main:
fn main(){

    let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)/cos(x)");
    //let function = String::from("sin(x)*e^(x)+cos(x)*ln(x)");
    //let function = String::from("3.56*x+7");
    
    // let start = Instant::now();
    // let duration = start.elapsed();
    // println!("Time spent: {:?}", duration);
    let list: Vec<Node> = vec_infix_to_postfix(string_to_vec_of_node(&function));
    
    

    for x in &list {
        x.print_value();
    }
    //generate_ir(&tree);
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
