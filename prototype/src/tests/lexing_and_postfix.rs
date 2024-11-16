use crate::stages::function_lexing::{
    lex_function,
    convert_infix_to_postfix
};

#[test]
fn gen_1(){
    let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)/cos(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();

    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "7.56,x,*,sin,x,1,+,e^,*,x,8,-,tg,x,cos,/,-,")
}

#[test]
fn gen_2(){
    let function = String::from("sin(x)*e^(x)+cos(x)*ln(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str.as_str(), "x,sin,x,e^,*,x,cos,x,ln,*,+,")
}

#[test]
fn gen_3(){
    let function = String::from("sin(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "x,sin,")
}

#[test]
fn gen_4(){
    let function = String::from("3.0*x+7.0");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "3,x,*,7,+,")
}

#[test]
fn gen_5(){
    let function = String::from("x*exp(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "x,x,e^,*,")
}

#[test]
fn gen_6(){
    let function = String::from("cos(x)-e^x-sin(x+7)+ln(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "x,cos,x,e^,-,x,7,+,sin,-,x,ln,+,")
}

#[test]
fn gen_7(){
    let function = String::from("1.89*x+x^2-3*x^7");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "1.89,x,*,x,2,^,+,3,x,7,^,*,-,")
}

#[test]
fn gen_8(){
    let function = String::from("sin(cos(e^(tg(3+x^2))))");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "3,x,2,^,+,tg,e^,cos,sin,")
}

#[test]
fn gen_9(){
    let function = String::from("x+9+x+x-x+2*x-x+8");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "x,9,+,x,+,x,+,x,-,2,x,*,+,x,-,8,+,")
}

#[test]
#[should_panic]
fn panic_gen_0(){
    let function = String::from("x*random(x)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }
}

#[test]
#[should_panic]
fn panic_gen_1(){
    let function = String::from("cus*x+x*x");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }
}