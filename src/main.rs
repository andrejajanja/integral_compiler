use std::{
    env,
    process::exit,
};
use integral_aprox::*;

struct Rem {
    start: usize,
    end: usize,
}

fn remove_lower_level(fun: &mut String){
    let mut pos: usize = 0;
    let mut remove_list = Vec::<Rem>::new();
    for (i, c) in fun.chars().enumerate(){
        if c == '(' {
            pos = i;
        }

        if c == ')' {
            remove_list.push(Rem{start: pos, end: i})
        }
    }
    
    pos = 0;
    for remove in &remove_list {
        fun.replace_range((remove.start - pos)..(remove.end - pos + 1), "");
        pos += remove.end - remove.start + 1;
    }
}

fn main() {
    let provided: Vec<String> = env::args().collect();
    if provided.len() != 1 {
        if provided[1] == "--help"{
            print_help();
            exit(0);
        }
    }

    let mut function = String::from("sin(x)*e^(x-1)-tg(x)/ln(x)");
    // let mut start: f64 = 0.5;
    // let mut end: f64 = 1.0;
    // let mut steps: u64 = 100;

    remove_lower_level(&mut function);

    println!("\n\n{function}\n\n");

    //parse_inputs(&mut function, &mut start, &mut end, &mut steps);
    //function = function.replace(" ", "");

    // sin(x) * e^(x+7) - tg(x) / ln(x*9)
    //println!("'{}'", function);
    //print!("\n\n  {}\n\n ∫ 3*x + 7 dx  =  89.0\t\t(With {steps} steps)\n\n{}\n\n", end, start);
    //let rez = integral(0.0, 3.14, 100000, &fun);
}
