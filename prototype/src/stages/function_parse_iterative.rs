#![allow(dead_code)]
use crate::components::object_type_definitions::*;
use crate::components::terminal_decoration::Color;
use crate::unrecoverable_error;
use std::process::exit;

use super::function_parse_recursive::print_tree_rec;

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

fn try_parsing(chunk: &str, function: &str) -> Option<Node> {
    match chunk.len() {
        1 => {
            match &chunk[..]{
                "*" => return Some(Node::from_func(Func::Mul)),
                "/" => return Some(Node::from_func(Func::Div)),
                "+" => return Some(Node::from_func(Func::Add)),
                "-" => return Some(Node::from_func(Func::Sub)),
                "^" => return Some(Node::from_func(Func::Pow)),
                "x" => return Some(Node::from_func(Func::X)),
                "(" => return Some(Node::from_func(Func::Ob)),
                ")" => return Some(Node::from_func(Func::Cb)),
                _ => return None
            }
        }
        2 => {
            match &chunk[..]{
                "ln" => return Some(Node::from_func(Func::Ln)),
                "e^" => return Some(Node::from_func(Func::Exp)),
                "tg" => return Some(Node::from_func(Func::Tg)),
                _ => return None
            }
        }
        3 => {
            match &chunk[..]{
                "sin" => return Some(Node::from_func(Func::Sin)),
                "cos" => return Some(Node::from_func(Func::Cos)),
                "ctg" => return Some(Node::from_func(Func::Ctg)),
                "atg" => return Some(Node::from_func(Func::Atg)),
                "exp" => return Some(Node::from_func(Func::Exp)),
                _ => return None
            }
        }
        4 => {
            match &chunk[..]{
                "sqrt" => return Some(Node::from_func(Func::Sqrt)),
                "asin" => return Some(Node::from_func(Func::Asin)),
                "acos" => return Some(Node::from_func(Func::Acos)),
                "atan" => return Some(Node::from_func(Func::Atg)),
                "actg" => return Some(Node::from_func(Func::Actg)),
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

pub fn parse_function(function: &str) -> Vec<Node> {
    let mut list: Vec<Node> = Vec::<Node>::new();

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

        let temp_node: Option<Node>;

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

            temp_node = Some(Node::from_value(temp_const));
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

fn in_op_priority(op: &Node) -> u8{
    match op.op {
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

fn st_op_priority(op: &Node) -> u8{
    match op.op {
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

pub fn convert_infix_to_postfix(infix: Vec<Node>) -> Vec<Node>{
    let mut postfix: Vec<Node> = Vec::<Node>::new();
    let mut stack: Vec<Node> = Vec::<Node>::new();

    let mut i: usize = 0;

    while i < infix.len() {
        if in_op_priority(&infix[i]) == 11 {
            postfix.push(infix[i].clone());
            i+=1;
            continue;
        }

        let mut stack_top: Option<Node> = match stack.pop() {
            Some(x) => {
                Some(x.clone())
            }
            None => {
                None
            }
        };

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
                None => {
                    break;
                }
            }
        }

        if infix[i].op != Func::Cb {
            stack.push(infix[i].clone());
        }else{            
            stack.pop();
        }        
        
        i+=1;
    }

    loop {
        let last_on_stack: Node = match stack.pop() {
            Some(nod) => {
                nod
            }

            None => {
                break;
            }
        };

        postfix.push(last_on_stack);
    }
    postfix
}

fn postfix_to_tree_verbose(list: &mut Vec<Node>) -> Node {
    //check if it's more efficient to format this differently
    let unary_ops = vec![Func::Sin,Func::Cos,Func::Tg,Func::Ctg,Func::Ln,Func::Exp,Func::Sqrt,Func::Atg,Func::Actg,Func::Asin,Func::Acos];
    match list.len() {
        0 => {
            panic!("Tree can't be generated due to list having no elements");
        }

        1 => {
            return list[0].clone();
        }

        2 => {
            list[1].left = Some(Box::new(list[0].clone()));
            list.remove(0);
        }
        _ => {
            let mut second: usize = 2;
            let mut first: usize = 1;
            let mut zeroth: usize = 0;
            let mut i: usize = 0;

            while list.len() > 2{ //this ensures that the queue is always longer than two elements
                println!("------------- {}th Passing, queue state - len: {}, zeroth {}", i, list.len(), zeroth);
                for temp in &mut *list{
                    print!("-> ");
                    print_tree_rec(temp, 0, '\n');
                }

                if unary_ops.contains(&list[first].op) && list[first].left.is_none(){
                    list[first].left = Some(Box::new(list[zeroth].clone()));
                    list.remove(zeroth);
                    i+=1;
                    continue;
                }

                if (list[second].op == Func::X || matches!(list[second].op, Func::Const(_))) && zeroth == 0{
                    zeroth+=1;
                    first+=1;
                    second+=1;
                }

                if zeroth == 1 && unary_ops.contains(&list[zeroth].op) && list[zeroth].left.is_none(){
                    list[1].left = Some(Box::new(list[0].clone()));
                    list.remove(0);
                    i+=1;
                    continue;
                }

                if unary_ops.contains(&list[second].op){
                    list[second].left = Some(Box::new(list[first].clone()));
                    list.remove(first);
                    i+=1;
                    continue;
                }
                
                list[second].left = Some(Box::new(list[first].clone()));
                list[second].right = Some(Box::new(list[zeroth].clone()));
                list.drain(zeroth..second);        

                if zeroth == 0{
                    if list.len() > 3{
                        zeroth+=1;
                        first+=1;
                        second+=1;
                    }
                }else{
                    zeroth-=1;
                    first-=1;
                    second-=1;
                }
                
                i+=1;
            }

            //In case two elements are left,
            //this means some unary operation is on the second place, thus it can be folded deterministicaly.
            if list.len() == 2{
                list[1].left = Some(Box::new(list[0].clone()));
                list.remove(0);
            }
        }
    }

    //println!("Lista {:?}, duzina {}", list, list.len());
    //print_tree(&list[0], 0, '\n');
    println!("----- ENDED TREE CREATION -----");
    list[0].clone()
}

fn postfix_to_tree(list: &mut Vec<Node>) -> Node {
    let unary_ops = vec![Func::Sin,Func::Cos,Func::Tg,Func::Ctg,Func::Ln,Func::Exp,Func::Sqrt,Func::Atg,Func::Actg,Func::Asin,Func::Acos];
    match list.len() {
        0 => {
            unrecoverable_error!(
                "Static analysis Error | Error occured in during conversion of postfix form to tree",
                "Tree can't be generated due to list having no elements."
            );
        }

        1 => {
            return list[0].clone();
        }

        2 => {
            //check if this thing works proprely
            list[0].left = Some(Box::new(list.remove(0)));
        }

        _ => {
            let mut second: usize = 2;
            let mut first: usize = 1;
            let mut zeroth: usize = 0;

            while list.len() > 2{ //this ensures that the queue is always longer than two elements
                if unary_ops.contains(&list[first].op) && list[first].left.is_none(){
                    list[zeroth].left = Some(Box::new(list.remove(zeroth)));
                    continue;
                }

                if (list[second].op == Func::X || matches!(list[second].op,Func::Const(_))) && zeroth == 0{
                    zeroth+=1;
                    first+=1;
                    second+=1;
                }

                if zeroth == 1 && unary_ops.contains(&list[zeroth].op) && list[zeroth].left.is_none(){
                    list[0].left = Some(Box::new(list.remove(0)));                    
                    continue;
                }

                if unary_ops.contains(&list[second].op){
                    list[first].left = Some(Box::new(list.remove(first)));                                     
                    continue;
                }
                
                list[first].left = Some(Box::new(list.remove(first)));
                list[zeroth].right = Some(Box::new(list.remove(zeroth)));

                if zeroth == 0{
                    if list.len() > 3{
                        zeroth+=1;
                        first+=1;
                        second+=1;
                    }
                }else{
                    zeroth-=1;
                    first-=1;
                    second-=1;
                }   
            }

            //In case two elements are left,
            //this means some unary operation is on the second place, thus it can be folded deterministicaly.
            if list.len() == 2{
                list[0].left = Some(Box::new(list.remove(0)));
            }
        }
    }
    list[0].clone()
}

pub fn str_to_tree_iter(function: &str) -> Node{
    let mut list: Vec<Node> = convert_infix_to_postfix(parse_function(function));
    let root = postfix_to_tree(&mut list);
    root
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
