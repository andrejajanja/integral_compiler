use crate::components::{
    object_type_definitions::Func,
    polynomials::TsPoly,
    taylor_optimizer::optimize_postfix_using_taylor
};

#[test]
fn seq_0(){
    let mut sequence = vec![Func::Poly(TsPoly::from_vec(vec![3.0, 1.0, 1.0], true)), Func::Const(10.0), Func::Add];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![13.0, 1.0, 1.0], true))]);
}

#[test]
fn seq_1(){
    let mut sequence = vec![Func::Poly(TsPoly::from_vec(vec![3.0, 1.0, 1.0], true)), Func::Const(10.0), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![30.0, 10.0, 10.0], true))]);
}

#[test]
fn seq_2(){
    let mut sequence = vec![Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Const(2.0), Func::Div];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 9);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![2.0, 1.0, 1.0], true))]);
}

#[test]
fn seq_3(){
    let mut sequence = vec![Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::X, Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![0.0, 4.0, 2.0, 2.0], true))]);
}

#[test]
fn seq_4(){
    let mut sequence = vec![Func::X, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![0.0, 4.0, 2.0, 2.0], true))]);
}

#[test]
fn seq_5(){
    let mut sequence = vec![Func::X, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Mul, Func::Const(5.0), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![0.0, 20.0, 10.0, 10.0], true))]);
}

#[test]
fn seq_6(){
    let mut sequence = vec![Func::X, Func::Poly(TsPoly::from_vec(vec![15.0, 10.0, 10.0], true)), Func::Mul, Func::Const(5.0), Func::Div];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![0.0, 3.0, 2.0, 2.0], true))]);
}

#[test]
fn seq_7(){
    let mut sequence = vec![Func::X, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Add];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 2);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![4.0, 3.0, 2.0], true))]);
}

#[test]
fn seq_8(){
    let mut sequence = vec![Func::X, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![0.0, 4.0, 2.0, 2.0], true))]);
}

#[test]
fn seq_9(){
    let mut sequence = vec![Func::X, Func::Const(1.0), Func::Add, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![4.0, 6.0, 4.0, 2.0], true))]);
}

#[test]
fn seq_10(){
    let mut sequence = vec![Func::X, Func::Const(1.0), Func::Add, Func::Const(2.0), Func::Div, Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)), Func::Mul];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 3);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![2.0, 3.0, 2.0, 1.0], true))]);
}

#[test]
fn seq_11(){
    let mut sequence = vec![
        Func::Poly(TsPoly::from_vec(vec![1.0, 1.0, 2.0], true)),
        Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)),
        Func::Mul
    ];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 4);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![4.0, 6.0, 12.0, 6.0, 4.0], true))]);
}

#[test]
fn seq_12(){
    let mut sequence = vec![
        Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 0.0, 0.0, 0.0, 2.0], true)),
        Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)),
        Func::Mul
    ];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 5);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![16.0, 16.0, 12.0, 4.0, 0.0, 8.0], true))]);
}

#[test]
fn seq_13(){
    let mut sequence = vec![
        Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 0.0, 0.0, 0.0, 2.0], true)),
        Func::Poly(TsPoly::from_vec(vec![4.0, 2.0, 2.0], true)),
        Func::Add
    ];
    optimize_postfix_using_taylor(&mut sequence, 0.0, 5);
    assert_eq!(sequence, vec![Func::Poly(TsPoly::from_vec(vec![8.0, 4.0, 2.0, 0.0, 0.0, 2.0], true))]);
}
