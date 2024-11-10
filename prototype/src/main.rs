#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]

mod components;
mod stages;

use prototype::stages::taylor_ir_compile::generate_taylor_ir;

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}
fn main(){
    generate_taylor_ir(&String::from("sin(x)*e^x+cos(x)*ln(x)"), 0.0, 9);
    // let mut a = TsPoly::from_func(Func::Exp, 1.5, 8);

    // let b = TsPoly::from_func(Func::Sin, 1.5, 8);
    // a.of(b);
    // a.truncate(10);
    // println!("y = {}", a);
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}