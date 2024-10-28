#![allow(temporary_cstring_as_ptr)]

mod components;
mod stages;

use std::{
    env::args,
    time::Instant,
    arch::x86_64::*
};

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}

#[target_feature(enable = "avx")]
unsafe fn simd_sin(x: __m256d) -> __m256d{
    let mut value = x;
    let xsq = _mm256_mul_pd(x, x);
    let x3rd = _mm256_mul_pd(xsq, x);
    let x5th = _mm256_mul_pd(x3rd, x);
    value = _mm256_fmadd_pd(x3rd, _mm256_set1_pd(-0.166666667), value);
    _mm256_fmadd_pd(x5th, _mm256_set1_pd(0.008333333), value)
}

#[target_feature(enable = "avx")]
unsafe fn do_simd_math(num: u64, adds: u64) -> __m256d{
    let mut result = _mm256_set1_pd(0.0);
    for _ in 0..num {
        let mut a = _mm256_set1_pd(3.14/4.0);
        // let b = _mm256_set1_pd(1.5);

        for _ in 0..adds{
            a = simd_sin(a);
        }
        result = a;        
    }
    result
}

fn main(){

    let args: Vec<String> = args().collect();

    let num = match args[1].parse::<u64>(){
        Ok(num) => num,
        Err(_) => panic!("BAD USIZE VALUE FOR LOOP ITERATIONS"),
    };

    let adds = match args[2].parse::<u64>(){
        Ok(num) => num,
        Err(_) => panic!("BAD F64 VALUE"),
    };

    let start = Instant::now();
    let result = unsafe {do_simd_math(num, adds)};

    let duration = start.elapsed();
    println!("Value: {:?}\n Elapsed: {:?}ns", result, duration.as_nanos()/(num*adds) as u128);
    
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}