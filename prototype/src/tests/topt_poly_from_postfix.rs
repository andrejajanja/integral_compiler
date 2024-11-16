use crate::{
    components::{
        polynomials::TsPoly,
        object_type_definitions:: Func,
        taylor_optimizer::optimize_postfix_using_taylor
    },
    stages::function_lexing::{lex_function, convert_infix_to_postfix}
};

#[test]
fn gen_0(){
    let mut sequence = lex_function(&String::from("x+9+x+x-x+2*x-x+8"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 1);

    if let Func::Poly(poly) = &sequence[0] {
        assert_eq!(poly.to_owned(), TsPoly::from_vec(vec![17.0, 3.0]));
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}

#[test]
fn gen_1(){
    let mut sequence = lex_function(&String::from("8-8+x-x+x-x+2*x-x"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 1);

    if let Func::Poly(poly) = &sequence[0] {
        assert_eq!(poly.to_owned(), TsPoly::from_vec(vec![0.0, 1.0]));
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}

#[test]
fn gen_2(){
    let mut sequence = lex_function(&String::from("x^3-2*x^2+7"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);

    if let Func::Poly(poly) = &sequence[0] {
        assert_eq!(poly.to_owned(), TsPoly::from_vec(vec![7.0, 0.0, -2.0, 1.0]));
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}

#[test]
fn gen_3(){
    let mut sequence = lex_function(&String::from("x^3+1-2*x^4+7"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 4);

    if let Func::Poly(poly) = &sequence[0] {
        assert_eq!(poly.to_owned(), TsPoly::from_vec(vec![8.0, 0.0, 0.0, 1.0, -2.0]));
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}

#[test]
fn gen_4(){
    let mut sequence = lex_function(&String::from("7+x-2*x^4"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 4);

    if let Func::Poly(poly) = &sequence[0] {
        assert_eq!(poly.to_owned(), TsPoly::from_vec(vec![7.0, 1.0, 0.0, 0.0, -2.0]));
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}

#[test]
fn gen_5(){
    let mut sequence = lex_function(&String::from("x^8"));
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, 0.0, 4);

    if let Func::Poly(poly) = &sequence[0] {
        let mut temp = TsPoly::zero();
        temp.max_pow = 4;
        assert_eq!(poly.to_owned(), temp);
    }else{
        panic!("Expected to have just one Func::Poly in the sequence, got something else");
    }
}