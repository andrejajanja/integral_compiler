use prototype::{
    stages::binary_compile::generate_function,
    components::auxilary_functions::parse_plot_input_file
};
use csv;

fn main() {
    println!("Started ploting");
    let plot_conf = parse_plot_input_file("./test_config.toml");
    let fja = generate_function(
        &plot_conf.function,
        plot_conf.precision_center, 
        plot_conf.poly_power
    );

    let mut wtr = csv::Writer::from_path(plot_conf.path).unwrap();    
    wtr.write_record(&["x", "y"]).unwrap();

    let mut cp = plot_conf.precision_center-plot_conf.epsilon;
    let step = 2.0*plot_conf.epsilon/(plot_conf.samples as f64);

    for _ in 0..plot_conf.samples {
        wtr.write_record(&[format!("{}", cp), format!("{}", fja(cp))]).unwrap();
        cp = cp + step;
    }

    wtr.flush().unwrap();
    println!("Finnished ploting");
}