use crate::{
    components::taylor_optimizer::optimize_postfix_using_taylor,
    stages::function_lexing::{lex_function, convert_infix_to_postfix}
};

#[test]
fn eval_0(){
    let function = String::from("exp(9)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", f64::exp(9.0)))
}

#[test]
fn eval_1(){
    let function = String::from("7.89+cos(11)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 7.89+f64::cos(11.0)))
}


#[test]
fn eval_2(){
    let function = String::from("tg(0.1)/10");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", f64::tan(0.1)/10.0))
}

#[test]
fn eval_3(){
    let function = String::from("4*ln(3)+7");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 4.0*f64::ln(3.0)+7.0))
}

#[test]
fn eval_4(){
    let function = String::from("3/sqrt(4)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 3.0/f64::sqrt(4.0)))
}

#[test]
fn eval_5(){
    let function = String::from("1+4*cos(5*e^7)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 1.0+4.0*f64::cos(5.0*f64::exp(7.0))))
}

#[test]
fn eval_6(){
    let function = String::from("1-4*cos(5*e^7-4)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 1.0-4.0*f64::cos(5.0*f64::exp(7.0)-4.0)))
}

#[test]
fn eval_7(){
    let function = String::from("8-sin(6)/cos(1)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 8.0-f64::sin(6.0)/f64::cos(1.0)))
}

#[test]
#[should_panic] //Panic for devision with zero
fn eval_8(){
    let function = String::from("8-cos(6)/sin(0)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "0.0,")
}

#[test]
#[should_panic] //Panic for invalid function domain
fn eval_9(){
    let function = String::from("8-cos(6)/ln(-1)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "0.0,")
}

#[test]
#[should_panic] //Panic for invalid function domain
fn eval_10(){
    let function = String::from("8-acos(2)/ln(10)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "0.0,")
}

#[test]
//#[should_panic] //Panic for invalid function domain
fn eval_11(){
    let function = String::from("8-acos(2)/ln(10)");
    let mut sequence = lex_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, "0.0,")
}