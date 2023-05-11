pub fn integral(a: f64, b: f64, steps: i64, _fun: &dyn Fn(f64) -> f64) -> f64 {
    if a>b{ 
        panic!("a value can't be bigger than b, see --help for instructions");
    };
    if a==b{
        return 0.0range()
    };
    let s: f64 = 0.0;
    let dx: f64 = (b-a)/(steps as f64);
    for i in (0..steps).rev(){

    }

    dx
}