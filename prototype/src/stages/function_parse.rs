#![allow(dead_code, unused_imports)]
use crate::components::object_type_definitions::*;
use crate::components::terminal_decoration::Color;
use crate::unrecoverable_error;
use std::process::exit;

pub fn tree_to_string_iter(root: &Node) -> String {
    let helper_root = root.clone();
    let mut content = String::from("");
    let mut st: Vec<Node> = Vec::<Node>::new();
    st.push(helper_root);
    loop {
        match st.pop(){
            Some(mut nd) => {
                loop{
                    content += &(nd.op.to_string() + ",");
                    match &nd.right {
                        Some(scnd) => {
                            st.push(*scnd.clone());
                        }
                        None => {

                        }
                    }
                    match &nd.left {
                        Some(x) => {
                            nd = *x.clone();
                            continue;
                        }
                        None => {
                            break;
                        }
                    } 
                }
            }
            None => {
                break;
            }
        }
    }
    content
}

fn try_parsing(chunk: &str, function: &str) -> Option<Func> {
    match chunk.len() {
        1 => {
            match &chunk[..]{
                "*" => return Some(Func::Mul),
                "/" => return Some(Func::Div),
                "+" => return Some(Func::Add),
                "-" => return Some(Func::Sub),
                "^" => return Some(Func::Pow),
                "x" => return Some(Func::X),
                "(" => return Some(Func::Ob),
                ")" => return Some(Func::Cb),
                _ => return None
            }
        }
        2 => {
            match &chunk[..]{
                "ln" => return Some(Func::Ln),
                "e^" => return Some(Func::Exp),
                "tg" => return Some(Func::Tg),
                _ => return None
            }
        }
        3 => {
            match &chunk[..]{
                "sin" => return Some(Func::Sin),
                "cos" => return Some(Func::Cos),
                "ctg" => return Some(Func::Ctg),
                "atg" => return Some(Func::Atg),
                "exp" => return Some(Func::Exp),
                _ => return None
            }
        }
        4 => {
            match &chunk[..]{
                "sqrt" => return Some(Func::Sqrt),
                "asin" => return Some(Func::Asin),
                "acos" => return Some(Func::Acos),
                "atan" => return Some(Func::Atg),
                "actg" => return Some(Func::Actg),
                _ => return None
            }
        }
        _ => {
            unrecoverable_error!(
                "Parsing Error | Highlighted part of a function string is unknown/unsupported function",
                &function.replace(&chunk, &format!("{}{} {} {}{}", Color::CBlack, Color::BYellow,&chunk, Color::CYellow, Color::BBlack))
            );
        }
    }
}

pub fn parse_function(function: &str) -> Vec<Func> {
    let mut list: Vec<Func> = Vec::<Func>::new();

    let mut i: usize = 0;
    let mut buffer: usize = 1;

    while i+buffer<function.len()+1{
        let mut temp = i;

        while let Some(ch) = function.chars().nth(temp) {
            if ch.is_digit(10) || ch == '.' {
                buffer+=1;
                temp+=1;
                continue;
            }
            break;
        }

        let temp_node: Option<Func>;

        if i == temp{
            //FIXME this is where some performance is lost due to elegant error handling
            temp_node = try_parsing(&function[i..i+buffer], &function);
        }else {
            buffer-=1;
            let temp_const = match function[i..i + buffer].parse::<f64>() {
                Ok(c) => c,
                Err(_c) => {
                    unrecoverable_error!("Parsing Error | Failed to parse a number in function string", &function[i..i + buffer]);
                }
            };

            temp_node = Some(Func::Const(temp_const));
        }

        match temp_node {
            Some(list_node) => {
                i += buffer;
                buffer = 1;
                list.push(list_node);
            }
            None => {
                buffer+=1;
            }
        }
    }

    list
}

fn in_op_priority(op: &Func) -> u8{
    match op{
        Func::Add | Func::Sub=> {
            return 2;
        }

        Func::Mul | Func::Div => {
            return 3;
        }
        
        Func::Pow => {
            return 5;
        }

        Func::Ob => {
            return 9;
        }

        Func::Cb => {
            return 1;
        }

        Func::Const(_) | Func::X=> {
            return 11;
        }

        //all other functions that behave as unary operators in this stack coversion
        _ => {
            return 8;
        }
    }
}

fn st_op_priority(op: &Func) -> u8{
    match op{
        Func::Add | Func::Sub=> {
            return 2;
        }

        Func::Mul | Func::Div => {
            return 3;
        }
        
        Func::Pow => {
            return 4;
        }

        Func::Ob => {
            return 0;
        }

        Func::Cb => {
            return 0;
        }

        _ => {
            return 7;
        }
    }
}

pub fn vec_node_to_string(ve: &Vec<Node>) -> String{
    let mut helper_string = String::new();

    for x in ve {
        helper_string += &(x.op.to_string() + " ");
    }

    helper_string + "\n"
}

pub fn convert_infix_to_postfix(infix: &mut Vec<Func>){
    let mut postfix: Vec<Func> = Vec::<Func>::new();
    let mut stack: Vec<Func> = Vec::<Func>::new();

    let mut i: usize = 0;

    while i < infix.len() {
        if in_op_priority(&infix[i]) == 11 {
            postfix.push(infix[i].clone());
            i+=1;
            continue;
        }

        let mut stack_top: Option<Func> = stack.pop();

        loop {
            match stack_top {
                Some(x) => {
                    if in_op_priority(&infix[i]) < st_op_priority(&x) {                                                                                            
                        postfix.push(x.clone());
                        stack_top = stack.pop();          
                    }else{
                        stack.push(x.clone());
                        break;
                    }
                }
                None => break
            }
        }

        if infix[i] != Func::Cb {
            stack.push(infix[i].clone());
        }else{            
            stack.pop();
        }        
        
        i+=1;
    }

    loop {
        let last_on_stack: Func = match stack.pop() {
            Some(element) => element,
            None => break
        };

        postfix.push(last_on_stack);
    }
    *infix = postfix;
}

fn find_unique_funcs_iter(root: &Node) -> Vec<Func>{
    let mut unique_funcs = Vec::<Func>::new();

    let helper_root = root.clone();
    let mut st: Vec<Node> = Vec::<Node>::new();
    st.push(helper_root);
    loop {
        match st.pop(){
            Some(mut nd) => {
                loop{
                    if !unique_funcs.contains(&nd.op) && matches!(nd.op,Func::Const(_)) && nd.op != Func::X{
                        unique_funcs.push(nd.op);
                    }

                    match &nd.right {
                        Some(scnd) => {
                            st.push(*scnd.clone());
                        }
                        None => {}
                    }
                    match &nd.left {
                        Some(x) => {
                            nd = *x.clone();
                            continue;
                        }
                        None => {
                            break;
                        }
                    } 
                }
            }
            None => {
                break;
            }
        }
    }

    unique_funcs
}

