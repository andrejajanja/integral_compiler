#![allow(dead_code)]
use std::process::exit;

use crate::parts::object_type_definitions::*;

fn split_by_ops(
    function: &String,
    op1: char,
    op2: char,
    mk1: Func,
    mk2: Func,
) -> (Vec<String>, Vec<Func>) {
    let mut tier_chunks = Vec::<String>::new();
    let mut tier_ops = Vec::<Func>::new();

    let mut depth: u8 = 0;
    let mut first: i16 = -1;
    let mut second: usize = 0;
    for (i, c) in function.chars().enumerate() {
        if depth != 0 {
            if c == '(' {
                depth += 1;
            }
            if c == ')' {
                depth -= 1;
            }
            continue;
        } else {
            if c == '(' {
                depth += 1;
                continue;
            }
        }

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

fn generate_stairs(chunks: &[String], ops: &[Func]) -> Node {
    let mut node = Node::new();
    if chunks.len() == 1 {
        node.first = Some(Box::new(str_to_tree_rec(&chunks[0])));
    } else {
        node.op = ops[0];
        node.first = Some(Box::new(str_to_tree_rec(&chunks[0])));
        if ops.len() == 1 {
            node.second = Some(Box::new(str_to_tree_rec(&chunks[1])));
        } else {
            node.second = Some(Box::new(generate_stairs(&chunks[1..], &ops[1..])));
        }
    }
    node
}

pub fn str_to_tree_rec(function: &String) -> Node {
    let mut sub_node = Node::new();
    let (first_tier_chunks, first_tier_ops) =
        split_by_ops(function, '+', '-', Func::Add, Func::Sub);

    //if there are some first tier ops
    if first_tier_chunks.len() != 1 {
        //case if there are any first tier operation in the function string
        sub_node.op = first_tier_ops[0];
        sub_node.first = Some(Box::new(str_to_tree_rec(&first_tier_chunks[0])));
        if first_tier_chunks.len() == 2 {
            //there are just 2 elements of 1st tier ops, processing the other one manualy
            sub_node.second = Some(Box::new(str_to_tree_rec(&first_tier_chunks[1])));
        } else {
            //there are more than 2 elements of 1st tier ops, running a tree algorithm
            sub_node.second = Some(Box::new(generate_stairs(
                &first_tier_chunks[1..],
                &first_tier_ops[1..],
            )));
        }
    } else {
        //there aren't any first tier ops
        let (second_tier_chunks, second_tier_ops) =
            split_by_ops(&first_tier_chunks[0], '*', '/', Func::Mul, Func::Div);

        //if there are some second tier ops
        if second_tier_chunks.len() != 1 {
            sub_node.op = second_tier_ops[0];
            sub_node.first = Some(Box::new(str_to_tree_rec(&second_tier_chunks[0])));

            if second_tier_chunks.len() == 2 {
                sub_node.second = Some(Box::new(str_to_tree_rec(&second_tier_chunks[1])));
            } else {
                //there are more than 2 elements of 2nd tier ops, running a tree algorithm
                sub_node.second = Some(Box::new(generate_stairs(
                    &second_tier_chunks[1..],
                    &second_tier_ops[1..],
                )));
            }
        } else {
            //There aren't any 2nd tier ops, checking for the single ops
            sub_node.second = None;
            if function == "x" {
                sub_node.first = None;
                sub_node.op = Func::X;
                sub_node.c = None;
                return sub_node;
            }
            match function.parse::<f64>() {
                Ok(c) => {
                    sub_node.first = None;
                    sub_node.op = Func::Const;
                    sub_node.c = Some(c);
                    return sub_node;
                }
                Err(_c) => {}
            }
            //this chunk isn't x or a number, so it is a complex function
            //extracting a lower level of this chunk
            let mut start: usize = 0;
            let mut end: usize = 0;
            let mut depth: u8 = 0;
            for (i, c) in function.chars().enumerate() {
                if c == '(' {
                    if depth == 0 {
                        start = i;
                    }
                    depth += 1;
                }

                if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        end = i;
                    }
                }
            }
            let lower_level = &function[start + 1..end].to_string();

            //determening the function type
            sub_node.c = None;
            match &function[0..start] {
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
                "atg" => {
                    sub_node.op = Func::Atg;
                }
                "actg" => {
                    sub_node.op = Func::Actg;
                }
                "asin" => {
                    sub_node.op = Func::Asin;
                }
                "arccos" => {
                    sub_node.op = Func::Acos;
                }
                "e^" => {
                    sub_node.op = Func::Exp;
                }
                "ln" => {
                    sub_node.op = Func::Ln;
                }
                _ => {
                    println!("\n\tError parsing the function part. Check for typos!\n\tExact part that caused this error: '{}'\n", first_tier_chunks[0]);
                    exit(0);
                }
            }
            //further processing lower level of this chunk
            sub_node.first = Some(Box::new(str_to_tree_rec(lower_level)));
        }
    }
    sub_node
}

