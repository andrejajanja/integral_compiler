use crate::stages::string_to_tree_iterative::{str_to_tree_iter, tree_to_string_iter};

#[test]
fn gen_1(){
    let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)/cos(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "-,/,cos,x,tg,-,Const,x,*,e^,+,Const,x,sin,*,x,Const,")
}

#[test]
fn gen_2(){
    let function = String::from("sin(x)*e^(x)+cos(x)*ln(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "+,*,ln,x,cos,x,*,e^,x,sin,x,")
}

#[test]
fn gen_3(){
    let function = String::from("sin(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "sin,x,")
}

#[test]
fn gen_4(){
    let function = String::from("3*x+7");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "+,Const,*,x,Const,")
}
