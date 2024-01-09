#![allow(dead_code)]
use std::{process::exit, fs::read_to_string, fs::File, io::Write};

use crate::parts::object_type_definitions::*;
use crate::parts::auxilary_functions::*;

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

fn get_perticular_funcs(node: &Node, funs: &mut Vec<Func>) {
    if !funs.contains(&node.op) &&  node.op != Func::Const && node.op != Func::X{
        funs.push(node.op); //you will maybe need to add .clone() here to op
    }
    match &node.first {
        Some(no) => {
            get_perticular_funcs(&no, funs);
        }
        None => {return;}
    }
    match &node.second {
        Some(no) => {
            get_perticular_funcs(&no, funs);
        }
        None => {return;}
    }
}

fn generate_dec_def(funs: &Vec<Func>) -> String{
    let mut code = String::new();

    for elem in funs {
        match elem {
            Func::Sin => {code = code + "declare double @llvm.sin.f64(double %Val)\n";},
            Func::Cos => {code = code + "declare double @llvm.cos.f64(double %Val)\n";},
            Func::Ln => {code = code + "declare double @llvm.log.f64(double %Val)\n";},
            Func::Exp => {code = code + "declare double @llvm.exp.f64(double %Val)\n";},
            Func::Pow => {code = code + "declare double @llvm.pom.f64(double %Val, double %Power)\n";},
            Func::Sqrt => { code = code + "declare double @llvm.sqrt.f64(double %Val)\n";}
            Func::Tg => {pnc_not_impl()},
            Func::Ctg => {pnc_not_impl()},
            Func::Atg => {pnc_not_impl()},
            Func::Actg => {pnc_not_impl()},
            Func::Asin => {pnc_not_impl()},
            Func::Acos => {pnc_not_impl()},
            Func::Add | Func::Div | Func::Sub | Func::Mul=> {}, //these are standard instructions
            _ => {
                println!("There was an error generating the declaration code
                for this function: {:?}", elem);
                exit(0);
            }
        }
    }
    code
}

fn compile_tree(node: &Node, var_num: &mut i16) -> Subseq {
    let mut rep: String = String::from(""); //IR representation
    let mut addr_f: i16 = 0; //first
    let mut addr_s: i16 = 0; //second
    let _first: String = String::from("");
    let _second: String = String::from("");

    match &node.first {
        Some(_no) => {
            *var_num = *var_num + 1;
            addr_f = *var_num;
            //first += compile_tree(no, var_num);
        },
        None => {},
    }

    match &node.second {
        Some(_no) => {
            *var_num = *var_num + 1;
            addr_s = *var_num;
            //second += compile_tree(no, var_num).as_str();
        },
        None => {},
    }

    match &node.op {
        Func::Sin => todo!(),
        Func::Cos => todo!(),
        Func::Tg => todo!(),
        Func::Ctg => todo!(),
        Func::Ln => todo!(),
        Func::Exp => todo!(),
        Func::Pow => todo!(),
        Func::Sqrt => todo!(),
        Func::Const => {
            match node.c {
                Some(c) => {
                    return Subseq::new(format!("{:.6e}", c), -1);
                },
                None => {
                    panic!("HOW CAN c BE NONE WHEN OP IS CONST");
                },
            }            
        },
        Func::Atg => todo!(),
        Func::Actg => todo!(),
        Func::Asin => todo!(),
        Func::Acos => todo!(),
        Func::Add => {
            rep += format!("%{} = fadd double %{}, %{}\n", *var_num, addr_f, addr_s).as_str();
        },
        Func::Sub => todo!(),
        Func::Mul => {rep += "fmul double";},
        Func::Div => todo!(),
        Func::X => {println!("Reached the X")},
        _ => {
            println!("Failed to compile tree node, operation that caused the error");
        },
    }

    Subseq::new(rep, 0)
}

pub fn generate_ir(node: &Node) {    
    let mut functions_to_define: Vec<Func> = Vec::<Func>::new();

    let mut var_num = 2;

    let mut rep: String = String::from("%2 = alloca double\nstore double %0, ptr %2");
    rep += compile_tree(node, &mut var_num).return_code().as_str();

    //maybe to implement this as a part of compile_tree function to reduce number of memory accesses
    get_perticular_funcs(node, &mut functions_to_define);
    let decs = generate_dec_def(&functions_to_define);
    println!("{}", decs);
    println!("{}", rep);

    let contents = read_to_string("IR_template.txt").expect("problem reading the file");
    //-put definitions string in the IR_code file
    let mut file = File::create("IR_code.ll").expect("Creating of an .ll file failed");

    file.write(contents.as_bytes()).expect("Failed writing to an .ll file");
}

