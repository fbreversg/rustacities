fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tupi = (500, 6.4, 1);

    let (x, y, z) = tupi;

    println!("The value of y is: {}", y);

    let five_hundred = tupi.0;

    let six_point_four = tupi.1;

    let one = tupi.2;
}