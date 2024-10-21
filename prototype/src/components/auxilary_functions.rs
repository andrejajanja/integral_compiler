#![allow(dead_code)]

use std::{
    io::stdin,
    process::exit,
    fs::read_to_string
};

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
    
    Made by Andreja Janković; Year 2024; E-mail: andrejajanja@gmail.com\n\n");
}

pub fn pnc_not_impl() {
    panic!("NOT YET IMPLEMENTED");
}

pub fn parse_inputs() -> (String, f64, f64, u64){
    let mut function = String::from("");

    println!("f(x) = ");
    stdin().read_line(&mut function).unwrap_or_else(|err| {
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

    let start = match provided[0].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range start argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[0]
            );
            exit(0);
        }
    };

    let end = match provided[1].parse::<f64>() {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\n\nError parsing range end argument: {e}\nthis is the value passed: '{}'\n\n",
                provided[1]
            );
            exit(0);
        }
    };

    let steps = match provided[2].parse::<u64>() {
        Ok(num) => num,
        Err(e) => {
            println!("\n\nError parsing number of steps argument: {e}\nthis is the value passed: '{}'\n\n", provided[2]);
            exit(0);
        }
    };

    (function, start, end, steps)
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config{
    integral_config: IntegralConfig
}

#[derive(Deserialize, Debug)]
pub struct IntegralConfig{
    pub function: String,
    pub range_start: f64,
    pub range_end: f64,
    pub samples: u64
}


pub fn parse_input_file(file_path: &str) -> IntegralConfig {
    let config_content = read_to_string(file_path).expect("Failed to read config file");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse contents of config file");
    return config.integral_config;
}