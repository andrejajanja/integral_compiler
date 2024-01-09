#![allow(dead_code)]

use std::{io::stdin, process::exit};

pub fn print_help() {
    println!("
    \t\tIntegral calculator user manual\n\n
    Options:
    --help -> prints this message\n

    Example call:\n
    integral_aproximator <- call an executable

    Input the function in the shape of:  sin(x)*e^(x+7)-tg(x)/ln(x-9)
    Set parameters:  0.0 1.0 1000
                      |   |   |
    range start ------^   |   |
    range end ------------^   |
    number of steps ----------^   

    ^^ This call aproximates an integral on a range from 0.0 to 1.0 with 1000 steps of approximation\n

    range start -> where the range starts (number, integer or a real number)
    range end -> where the range ends (number, integer or a real number)
    number of steps -> integrer of steps on which the intgeral of a function is aproximated\n\t(recommended value is between 10^4 and 10^7)
    
    Made by Andreja Janković; Year 2023; E-mail: andrejajanja@gmail.com\n\n");
}

pub fn pnc_not_impl() {
    panic!("NOT YET IMPLEMENTED");
}

pub fn parse_inputs(function: &mut String, start: &mut f64, end: &mut f64, steps: &mut u64) {
    println!("f(x) = ");
    stdin().read_line(function).unwrap_or_else(|err| {
        println!("\n\nError while taking a function input: {err}\n\n");
        exit(0);
    });
    function.pop(); //remove the newline character

    let mut parameters: String = String::new();
    print!("\nrange start, range end, step count: \n");
    stdin().read_line(&mut parameters).unwrap_or_else(|err| {
        println!("\n\nError while taking a parameter input: {err}\n\n");
        exit(0);
    });

    parameters.pop();
    let provided: Vec<&str> = parameters.split(" ").collect();

    if provided.len() != 3 {
        println!("Parameters entered in insufficient format, please try again.");
        exit(0);
    }

    *start = match provided[0].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range start argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[0]
            );
            exit(0);
        }
    };

    *end = match provided[1].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range end argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[1]
            );
            exit(0);
        }
    };

    *steps = match provided[2].parse::<u64>() {
        Ok(num) => num,
        Err(e) => {
            println!("\n\nError parsing number of steps argument: {e}\nthis is the value passed: '{}'\n\n", provided[2]);
            exit(0);
        }
    };
}

// #[macro_export()]
// macro_rules! measure_time {
//     ($code:block) => {{
//         let start = Instant::now();
//         let result = $code;
//         let duration = start.elapsed();

//         println!("Time spent: {:?}", duration);
//         result
//     }};
// }

// fn _integral(a: f64, b: f64, steps: i64, fun: fn(f64) -> f64) -> f64 {
//     if a > b {
//         panic!("a value can't be bigger than b, see --help for instructions");
//     };
//     if a == b {
//         return 0.0;
//     };
//     let mut s: f64 = 0.0;
//     let dx: f64 = (b - a) / (steps as f64);
//     for i in (0..steps).rev() {
//         s += fun(a + (i as f64) * dx) * dx
//     }
//     s
// }

// fn _fun(x: f64) -> f64 {
//     x.cos()
// }
