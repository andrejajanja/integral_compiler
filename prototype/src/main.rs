#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]

mod components;
mod stages;

use prototype::components::object_type_definitions::Func;
use prototype::stages::polynomials::TsPoly;

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}
fn main(){

    let a  = TsPoly::from_func(Func::Ln, 3.0, 9);
    println!("y = {}", a);
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}