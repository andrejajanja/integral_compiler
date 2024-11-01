use crate::unrecoverable_error;
use crate::components::terminal_decoration::Color;
use std::process::exit;
use std::ops::{Add, AddAssign, Mul, Index, IndexMut};

/// # Arguments
/// - `poly1` - a first polynomial to be multiplied
/// - `poly2` - a second polynomial to be multiplied
/// 
/// # Result
/// The result of the multiplication is stored in the first argument (`poly1`)
#[derive(Debug, Clone)]
pub struct TsPoly {
    coefs: Vec<f64>,
    offset: f64
}

impl TsPoly{
    pub fn new() -> TsPoly{
        TsPoly { coefs: vec![0.0; 30], offset: 0.0}
    }
}

impl Add for TsPoly{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for TsPoly{
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}