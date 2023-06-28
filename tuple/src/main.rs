fn main() {
    let tup = (900, 6.4, 1);

    let (x, y, z) = tup;

    println!("Значение z = {}", z);

    let nine_hundred = tup.0;

    let one = tup.2;

    println!("{} {}", nine_hundred, one);
}
