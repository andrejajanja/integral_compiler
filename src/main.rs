mod library; use library::*;

fn fun(x: f64) -> f64{
    x.sin()
}

fn main() {
    let rez = integral(10.4, 44.6, 1000, &fun);
    print!("{rez}\n");
}
