use std::{
    process::exit,
    io::stdin,
};

fn _integral(a: f64, b: f64, steps: i64, fun: fn(f64) -> f64) -> f64 {
    if a>b{ 
        panic!("a value can't be bigger than b, see --help for instructions");
    };
    if a==b{
        return 0.0;
    };
    let mut s: f64 = 0.0;
    let dx: f64 = (b-a)/(steps as f64);
    for i in (0..steps).rev(){
        s+= fun(a + (i as f64)*dx)*dx
    }
    s
}

fn _fun(x: f64) -> f64{
    x.cos()
}

pub fn print_help(){
    println!("
    \t\tIntegral calculator user manual\n\n
    Options:
    --help -> prints this message\n

    Example call:\n
    integral_aproximator <- here you just call an executable

    Then you input the function in the shape of:  sin(x) * e^(x+7) - tg(x) / ln(x - 9)
    Then you set parameters:  0.0 1.0 1000
                                |   |   |
                    range start ^   |   |
                          range end ^   |
                        number of steps ^   

    ^^ This call aproximates an integral on a range from 0.0 to 1.0 with 1000 steps of approximation\n

    range start -> where the range starts (number, integer or a real number)
    range end -> where the range ends (number, integer or a real number)
    number of steps -> integrer of steps on which the intgeral of a function is aproximated\n\t(recommended value is between 10^4 and 10^7)
    
    Made by Andreja Janković; Year 2023; E-mail: andrejajanja@gmail.com\n\n");    
}

pub fn parse_inputs(function: &mut String, start: &mut f64, end: &mut f64, steps: &mut u64 ){

    println!("f(x) = ");
    stdin().read_line(function).unwrap_or_else(|err| {
        println!("\n\nError while taking a function input: {err}\n\n");
        exit(0);
    });
    function.pop();

    let mut parameters: String = String::new();
    print!("\nrange start, range end, step count: \n");
    stdin().read_line(&mut parameters).unwrap_or_else(|err| {
        println!("\n\nError while taking a parameter input: {err}\n\n");
        exit(0);
    });

    
    parameters.pop();
    let provided: Vec<&str> = parameters.split(" ").collect();

    if provided.len() != 3 {
        println!("Insufficient parameters entered, try again.");
        exit(0);
    }

    *start = match provided[0].parse::<f64>() {
        Ok(x) => {x}
        Err(e) => {
            println!("\n\nError parsing range start argument: {e}\nthis is the value passed: '{}'\n\n", provided[0]);
            exit(0);
        }
    };
    
    *end = match provided[1].parse::<f64>() {
        Ok(x) => {x}
        Err(e) => {
            println!("\n\nError parsing range end argument: {e}\nthis is the value passed: '{}'\n\n", provided[1]);
            exit(0);
        }
    };

    *steps = match provided[2].parse::<u64>() {
        Ok(num) => {num}
        Err(e) => {
            println!("\n\nError parsing number of steps argument: {e}\nthis is the value passed: '{}'\n\n", provided[2]);
            exit(0);
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Func {
    Sin,    // sin(f(x))
    Cos,    // cos(f(x))
    Tg,     // tg(f(x))
    Ctg,    // ctg(f(x))
    Ln,     // ln(f(x))
    Exp,    // e^(f(x))
    Pow,    // C^(f(x)) CeR
    Sqrt,   // sqrt(f(x))
    Const,  // C where CeR
    Arctg,  // arctg(f(x))
    Arcctg, // arcctg(f(x))
    Arcsin, // arcsin(f(x))
    Arccos, // arccos(f(x))
    //These are the operation +, -, *, /
    Add,
    Sub,
    Mul,
    Div,
    X, //if node is just an X
    //None is used to mark the end of node tree branches
    None
}

pub struct Node {
    pub first: Option<Box<Node>>,
    pub second: Option<Box<Node>>,
    pub op: Func,
}

impl Node {
    pub fn new() -> Node{
        Node{first: None, second: None, op: Func::None}
    }

    pub fn _parse_to_assebly(&self) {}
}

pub fn extract_lower_level(fun: &mut String) -> Vec<String>{
    struct Rem {
        start: usize,
        end: usize,
    }

    let mut pos: usize = 0;
    let mut depth: u8 = 0;
    let mut remove_list = Vec::<Rem>::new();
    for (i, c) in fun.chars().enumerate(){
        if c == '(' {
            if depth == 0{
                pos = i;
            }
            
            depth += 1; //it doesn't have an increment operator?
        }

        if c == ')' {
            depth -= 1;

            if depth == 0{
                remove_list.push(Rem{start: pos, end: i});
            }
        }
    }
    
    let mut lower_level = Vec::<String>::new();
    pos = 0;
    for remove in &remove_list {
        let pom = fun.clone(); 
        lower_level.push(pom[(remove.start - pos + 1)..(remove.end - pos)].to_string());

        fun.replace_range((remove.start - pos)..(remove.end - pos + 1), "");
        pos += remove.end - remove.start + 1;
    }

    lower_level
}

pub fn generate_tree_from_string(function: &mut String, node: &mut Node){
    let lower_level = extract_lower_level(function);

    //this is for spliting by + and -
    let mut first_tier_chunks = Vec::<String>::new();
    let mut first_tier_ops = Vec::<Func>::new();

    //splitting by first tier operations
    let mut first: i16 = -1;
    let mut second: usize = 0;
    for (i, c) in function.chars().enumerate() {
        if c == '+' || c == '-' {

            match c {
                '+' => {first_tier_ops.push(Func::Add);}
                '-' => {first_tier_ops.push(Func::Sub);}
                _ => {}
            }

            if second != 0 {
                first = second as i16;
            }
            second = i;
            first_tier_chunks.push(function[(first+1) as usize..second].to_string())
        }
    }
    
    if second == 0 {
        first_tier_chunks.push(function.clone());
    }else{
        first_tier_chunks.push(function[second+1..].to_string());
    }

    println!("First tier operations in level:");
    println!("\n{:#?}", first_tier_chunks);
    println!("\n{:#?}", first_tier_ops);

    if first_tier_ops.len() == 1 {
        //singular assingment
        node.op = first_tier_ops[0];
    }else{
        //staircase assingment with nodes
        
    }

    //spliting by second order operation, for every first tier chunk
    for (i, chunk) in first_tier_chunks.iter().enumerate(){
        //this is for splitting by * and /
        let mut second_tier_chunks = Vec::<String>::new();
        let mut second_tier_ops = Vec::<Func>::new();
        first = -1;
        second = 0;
        for (i, c) in chunk.chars().enumerate() {
            if c == '*' || c == '/' {
                match c {
                    '*' => {second_tier_ops.push(Func::Mul);}
                    '/' => {second_tier_ops.push(Func::Div);}
                    _ => {}
                }
    
                if second != 0 {
                    first = second as i16;
                }
                second = i;
                second_tier_chunks.push(chunk[(first+1) as usize..second].to_string())
            }
        }
        if second == 0 {
            second_tier_chunks.push(chunk.clone());
        }else{
            second_tier_chunks.push(chunk[second+1..].to_string());
        }

        println!("Second tier operations for chunk number: {i}");
        println!("\n{:#?}", second_tier_chunks);
        println!("{:#?}\n", second_tier_ops);

        //you need to take into account the staircase thing here
    }

    println!("\n{:#?}", lower_level);
}
