#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;
use prototype::{
    components::auxilary_functions::parse_plot_input_file,
    stages::{binary_compile::generate_function, custom_ir_compile::generate_custom_function, ir_compile::generate_ir}
};

fn main() {
    println!("\nStarted time benchmark");
    let plot_conf = parse_plot_input_file("./test_config.toml");
    let x: f64 = plot_conf.precision_center+0.1;
    
    let fja = generate_function(&plot_conf.function, plot_conf.precision_center, plot_conf.poly_power);

    let mut avg = 0.0;

    for _ in 0..plot_conf.samples {
        let ruler = unsafe{_rdtsc()};
        let _temp_x = fja(x);
        avg += (unsafe{_rdtsc()} - ruler) as f64;
    }

    println!("\nMy approach => average {:.4} cycles", avg/(plot_conf.samples as f64));

    let x: f64 = plot_conf.precision_center+0.1;
    
    let fja = generate_custom_function(generate_ir(&plot_conf.function));
    
    // let mut times: Vec<u64> = vec![0; samples];
    avg = 0.0;

    for _ in 0..plot_conf.samples {
        let ruler = unsafe{_rdtsc()};
        let _temp_x = fja(x);
        avg += (unsafe{_rdtsc()} - ruler) as f64;
    }
    

    println!("glibc => average {:.4} cycles\n", avg/(plot_conf.samples as f64));
}