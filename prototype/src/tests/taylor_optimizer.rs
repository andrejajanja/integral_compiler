use crate::stages::{
    taylor_ir_compile::optimize_postfix_using_tylor,
    function_parse::{parse_function,convert_infix_to_postfix}
};

#[test]
fn static_eval_0(){
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
fn static_eval_1(){
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
fn static_eval_2(){
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
fn static_eval_3(){
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
fn static_eval_4(){
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
fn static_eval_5(){
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

fn static_eval_6(){
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
fn static_eval_7(){
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