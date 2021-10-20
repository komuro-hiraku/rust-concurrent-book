fn main() {
    my_func4();
}

fn mul_x(x: u64) -> Box::<dyn Fn(u64) -> u64> {
    Box::new(move |y| x * y)
}

fn my_func4() {
    let f = mul_x(3);
    println!("f(5)={}", f(5));
}
