#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]

mod components;
mod stages;

use prototype::components::object_type_definitions::Func;
use prototype::components::polynomials::TsPoly;

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}
fn main(){

    let mut a = TsPoly::from_func(Func::Exp, 1.5, 8);

    let b = TsPoly::from_func(Func::Sin, 1.5, 8);
    a.of(b);
    a.truncate(10);
    println!("y = {}", a);
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}