#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
#![allow(unused_imports)]
mod components;
mod stages;

use crate::stages::binary_compile::{generate_binary_from_ir, save_generated_binary_to_file, generate_function};
use crate::stages::ir_compile::generate_ir;
use crate::stages::custom_ir_compile::generate_custom_function;
//use std::time::Instant;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;


// let samples = 1000000;

//     let mut times: Vec<u64> = vec![0; samples];

//     for index in 0..samples {
//         let ruler = unsafe{_rdtsc()};
//         let _temp_x = fja(x);
//         times[index] = unsafe{_rdtsc()} - ruler;
//     }

//     let mut avg = 0.0;
//     for sample in &times {
//         avg += *sample as f64;
//     }
//     avg /= samples as f64;

//     let mut devition = 0.0;
//     for sample in &times {
//         devition += (*sample as f64 - avg).powi(2);
//     }

//     devition/=(samples-1) as f64;
//     devition = devition.sqrt();

//     println!("Approach: Avg {:.4} Dev {:.4}", avg, devition);


fn measure_approach() {
    let x: f64 = 1.0;
    
    let fja = generate_function("sin(x)*exp(x)", 0.9, 8);

    let samples = 100_000_000;
    // let mut times: Vec<u64> = vec![0; samples];
    let mut avg = 0.0;

    for _ in 0..samples {
        let ruler = unsafe{_rdtsc()};
        let _temp_x = fja(x);
        avg += (unsafe{_rdtsc()} - ruler) as f64;
    }
    
    println!("Approach: Avg {:.4}", avg/(samples as f64));
}

fn measure_glibc() {
    let x: f64 = 1.0;
    
    let fja = generate_custom_function(generate_ir("sin(x)*exp(x)"));
    
    let samples = 100_000_000;
    // let mut times: Vec<u64> = vec![0; samples];
    let mut avg = 0.0;

    for _ in 0..samples {
        let ruler = unsafe{_rdtsc()};
        let _temp_x = fja(x);
        avg += (unsafe{_rdtsc()} - ruler) as f64;
    }
    

    println!("glibc: Avg {:.4}", avg/(samples as f64));
}

fn main(){
    measure_approach();
    measure_glibc();
}

//     let _ir_code = String::from(r"define double @fja(double %x, double* %array_ptr){
// %tpow1_0 = fmul double %x, %x
// %e1 = getelementptr double, double* %array_ptr, i64 1
// store double %tpow1_0, double* %e1

// %tpow2_0 = fmul double %tpow1_0, %x
// %e2 = getelementptr double, double* %array_ptr, i64 2
// store double %tpow2_0, double* %e2

// %tpow3_0 = fmul double %tpow2_0, %x
// %e3 = getelementptr double, double* %array_ptr, i64 3
// store double %tpow3_0, double* %e3

// %tpow4_0 = fmul double %tpow3_0, %x
// %e4 = getelementptr double, double* %array_ptr, i64 4
// store double %tpow4_0, double* %e4

// %tpow5_0 = fmul double %tpow4_0, %x
// %e5 = getelementptr double, double* %array_ptr, i64 5
// store double %tpow5_0, double* %e5

// %tpow6_0 = fmul double %tpow5_0, %x
// %e6 = getelementptr double, double* %array_ptr, i64 6
// store double %tpow6_0, double* %e6

// %tpow7_0 = fmul double %tpow6_0, %x
// %e7 = getelementptr double, double* %array_ptr, i64 7
// store double %tpow7_0, double* %e7

// ret double %tpow7_0
// }
// ");

    // let fja = generate_custom_function(ir_code);
    // let ptr = buffer.as_mut_ptr();

// let mut sequence = lex_function(&String::from("8-cos(6)/sin(0)"));
    // convert_infix_to_postfix(&mut sequence);
    // optimize_postfix_using_taylor(&mut sequence, 0.0, 9);