use crate::stages::{
    function_parse::{convert_infix_to_postfix, parse_function},
    taylor_ir_compile::optimize_postfix_using_tylor
};

#[test]
fn eval_0(){
    let function = String::from("exp(9)");
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

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
    let mut sequence = parse_function(&function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_tylor(&mut sequence, 0.0, 9);

    let mut temp_str = String::new();
    for elem in sequence {
        temp_str += &elem.to_string();
        temp_str += ",";
    }

    assert_eq!(temp_str, format!("{},", 8.0-f64::sin(6.0)/f64::cos(1.0)))
}