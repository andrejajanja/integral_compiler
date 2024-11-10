use crate::stages::function_parse_iterative::{str_to_tree_iter, tree_to_string_iter};

#[test]
fn gen_1(){
    let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)/cos(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "-,/,cos,x,tg,-,8,x,*,e^,+,1,x,sin,*,x,7.56,")
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
    let function = String::from("3.0*x+7.0");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "+,7,*,x,3,")
}

#[test]
fn gen_5(){
    let function = String::from("x*exp(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "*,e^,x,x,")
}

#[test]
#[should_panic]
fn gen_6(){
    let function = String::from("x*random(x)");
    let root = str_to_tree_iter(&function);
    assert_eq!(tree_to_string_iter(&root), "*,e^,x,x,")
}