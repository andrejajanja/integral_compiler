use std::{io::stdin, process::exit};

#[macro_export()]
macro_rules! measure_time {
    ($code:block) => {{
        let start = Instant::now();
        let result = $code;
        let duration = start.elapsed();

        println!("Time spent: {:?}", duration);
        result
    }};
}

fn _integral(a: f64, b: f64, steps: i64, fun: fn(f64) -> f64) -> f64 {
    if a > b {
        panic!("a value can't be bigger than b, see --help for instructions");
    };
    if a == b {
        return 0.0;
    };
    let mut s: f64 = 0.0;
    let dx: f64 = (b - a) / (steps as f64);
    for i in (0..steps).rev() {
        s += fun(a + (i as f64) * dx) * dx
    }
    s
}

fn _fun(x: f64) -> f64 {
    x.cos()
}

pub fn print_help() {
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

pub fn parse_inputs(function: &mut String, start: &mut f64, end: &mut f64, steps: &mut u64) {
    println!("f(x) = ");
    stdin().read_line(function).unwrap_or_else(|err| {
        println!("\n\nError while taking a function input: {err}\n\n");
        exit(0);
    });
    function.pop(); //remove the newline character here

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
    X, //if node is just x
    None,
}

#[derive(Debug)]
pub struct Node {
    pub first: Option<Box<Node>>,
    pub second: Option<Box<Node>>,
    pub op: Func,
    pub c: Option<f64>, //if type op = Func::Const
}

impl Node {
    pub fn new() -> Node {
        Node {
            first: None,
            second: None,
            op: Func::None,
            c: None,
        }
    }
}

pub fn extract_lower_level(fun: &mut String) -> Vec<String> {
    struct Rem {
        start: usize,
        end: usize,
    }

    let mut pos: usize = 0;
    let mut depth: u8 = 0;
    let mut remove_list = Vec::<Rem>::new();
    for (i, c) in fun.chars().enumerate() {
        if c == '(' {
            if depth == 0 {
                pos = i;
            }
            depth += 1;
        }
        if c == ')' {
            depth -= 1;

            if depth == 0 {
                remove_list.push(Rem { start: pos, end: i });
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

fn split_by_ops(
    function: &String,
    op1: char,
    op2: char,
    mk1: Func,
    mk2: Func,
) -> (Vec<String>, Vec<Func>) {
    let mut tier_chunks = Vec::<String>::new();
    let mut tier_ops = Vec::<Func>::new();

    let mut first: i16 = -1;
    let mut second: usize = 0;
    for (i, c) in function.chars().enumerate() {
        if c == op1 || c == op2 {
            if c == op1 {
                tier_ops.push(mk1);
            } else {
                tier_ops.push(mk2);
            }

            if second != 0 {
                first = second as i16;
            }
            second = i;
            tier_chunks.push(function[(first + 1) as usize..second].to_string())
        }
    }

    if second == 0 {
        tier_chunks.push(function.clone());
    } else {
        tier_chunks.push(function[second + 1..].to_string());
    }

    (tier_chunks, tier_ops)
}

fn parse_and_fill(pom_node: &mut Node, pom_string: &str) {
    if pom_string == "x" {
        pom_node.first = None;
        pom_node.second = None;
        pom_node.op = Func::X;
        pom_node.c = None;
    } else {
        match pom_string.parse::<f64>() {
            Ok(c) => {
                pom_node.first = None;
                pom_node.second = None;
                pom_node.op = Func::Const;
                pom_node.c = Some(c);
            }
            Err(_e) => {
                pom_node.first = None;
                pom_node.second = None;
                pom_node.c = None;

                match pom_string {
                    "sin" => {
                        pom_node.op = Func::Sin;
                    }
                    "cos" => {
                        pom_node.op = Func::Cos;
                    }
                    "tg" => {
                        pom_node.op = Func::Tg;
                    }
                    "ctg" => {
                        pom_node.op = Func::Ctg;
                    }
                    "sqrt" => {
                        pom_node.op = Func::Sqrt;
                    }
                    "arctg" => {
                        pom_node.op = Func::Arctg;
                    }
                    "arcctg" => {
                        pom_node.op = Func::Arcctg;
                    }
                    "arcsin" => {
                        pom_node.op = Func::Arcsin;
                    }
                    "arccos" => {
                        pom_node.op = Func::Arccos;
                    }
                    "e^" => {
                        pom_node.op = Func::Exp;
                    }
                    "ln" => {
                        pom_node.op = Func::Ln;
                    }
                    _ => {
                        println!("\n\tError parsing the function part. Check for typos\n\tExact part that caused this error: '{}'\n", pom_string);
                        exit(0);
                    }
                }
            }
        }
    }
}

fn generate_stairs(chunks: &[String], ops: &[Func], mut lower_level: &mut [String]) -> Node {
    let mut node = Node::new();
    if chunks.len() == 1 {
        parse_and_fill(&mut node, chunks[0].as_str());
        if lower_level.len() == 1 {
            node.first = Some(Box::new(generate_tree_from_string(&mut lower_level[0])));
        }
    } else {
        node.op = ops[0];
        let mut helper = Node::new();
        parse_and_fill(&mut helper, chunks[0].as_str());

        match helper.op {
            Func::X | Func::Const => {}
            _ => {
                helper.first = Some(Box::new(generate_tree_from_string(&mut lower_level[0])));
                lower_level = &mut lower_level[1..];
            }
        }
        node.first = Some(Box::new(helper));

        if ops.len() == 1{
            helper = Node::new();
            parse_and_fill(&mut helper, chunks[1].as_str());
            match helper.op {
                Func::X | Func::Const => {}
                _ => {
                    helper.first = Some(Box::new(generate_tree_from_string(&mut lower_level[0])));
                }
            }
            node.second = Some(Box::new(helper));

        }else{
            node.second = Some(Box::new(generate_stairs(&chunks[1..], &ops[1..], lower_level)));
        }
    }
    node
}

//println!("First tier operations:\n{:#?}\n\n{:#?}", first_tier_ops, first_tier_chunks);
pub fn generate_tree_from_string(function: &mut String) -> Node {
    let mut lower_level = extract_lower_level(function);
    let mut tracker: u8 = 0; //for lower level, remove this variable in favour of recursion

    let mut sub_node = Node::new();
    //this is for spliting by + and - <= First tier operations
    let (first_tier_chunks, first_tier_ops) =
        split_by_ops(function, '+', '-', Func::Add, Func::Sub);

    if first_tier_chunks.len() != 1 {
        //case if there are any first tier operation in the function string
        sub_node.op = first_tier_ops[0];

        let mut help_node = Node::new();
        parse_and_fill(&mut help_node, first_tier_chunks[0].as_str());
        let help_lower: &mut [String];
        match help_node.op {
            Func::X | Func::Const => {
                help_lower = &mut lower_level[..];
            }
            _ => {
                help_node.first =
                    Some(Box::new(generate_tree_from_string(&mut lower_level[0])));
                help_lower = &mut lower_level[1..];
            }
        }
        sub_node.first = Some(Box::new(help_node));

        if first_tier_chunks.len() == 2 {
            help_node = Node::new();
            parse_and_fill(&mut help_node, first_tier_chunks[1].as_str());
            match help_node.op {
                Func::X | Func::Const => {}
                _ => {
                    help_node.first =
                        Some(Box::new(generate_tree_from_string(&mut help_lower[0])));
                }
            }
            sub_node.second = Some(Box::new(help_node));
        } else {
            sub_node.second = Some(Box::new(generate_stairs(
                &first_tier_chunks[1..],
                &first_tier_ops[1..],
                help_lower,
            )));
        }
    } else {


        //OVDE MOZE DA SE IMPLEMENTIRA REKURZIJA OVE VELIKE FUNKCIJE DA BI SE SMANJILO KOPIRANJE CODE-a

        //here the number of chunks is one;
        let (second_tier_chunks, second_tier_ops) =
                    split_by_ops(&first_tier_chunks[0], '*', '/', Func::Mul, Func::Div);

        //There aren't any 1st tier ops in funcrion, checking for 2nd tier ops and running a tree for them
        if second_tier_chunks.len() == 1 {

            //There aren't any 2nd tier ops in function, then checks for the single op
            if lower_level.len() == 0 {
                parse_and_fill(&mut sub_node, function);
            } else {
                sub_node.first = None;
                sub_node.second = None;
                sub_node.c = None;
                match first_tier_chunks[0].as_str() {
                    "sin" => {
                        sub_node.op = Func::Sin;
                    }
                    "cos" => {
                        sub_node.op = Func::Cos;
                    }
                    "tg" => {
                        sub_node.op = Func::Tg;
                    }
                    "ctg" => {
                        sub_node.op = Func::Ctg;
                    }
                    "sqrt" => {
                        sub_node.op = Func::Sqrt;
                    }
                    "arctg" => {
                        sub_node.op = Func::Arctg;
                    }
                    "arcctg" => {
                        sub_node.op = Func::Arcctg;
                    }
                    "arcsin" => {
                        sub_node.op = Func::Arcsin;
                    }
                    "arccos" => {
                        sub_node.op = Func::Arccos;
                    }
                    "e^" => {
                        sub_node.op = Func::Exp;
                    }
                    "ln" => {
                        sub_node.op = Func::Ln;
                    }
                    _ => {
                        println!("\n\tError parsing the function part. Check for typos\n\tExact part that caused this error: '{}'\n", first_tier_chunks[0]);
                        exit(0);
                    }
                }
                sub_node.first = Some(Box::new(generate_tree_from_string(&mut lower_level[0])));
            }
        
        }else{

        }
    }
    //println!("\nLower level:\n{:#?}\n\n", lower_level);
    sub_node
}

pub fn print_tree(node: &Node, tab: usize, addition: char) {
    match &node.op {
        Func::Const => {
            print!("{}| {:?} |{}", "\t".repeat(tab), node.c, addition);
        }
        _ => {
            print!("{}| {:?} |{}", "\t".repeat(tab), node.op, addition);
        }
    }
    match &node.first {
        Some(no) => {
            match &node.second {
                None => {
                    print!("\n");
                }
                Some(_x) => {}
            }

            print_tree(&no, tab + 1, '\n');
        }
        None => {}
    }
    match &node.second {
        Some(no) => {
            print_tree(&no, tab + 1, '\n');
        }
        None => {}
    }
}
