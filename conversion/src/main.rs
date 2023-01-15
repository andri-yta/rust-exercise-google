fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    let xi16: i16 = x.into();

    println!("{x} * {y} = {}", multiply(xi16, y));
}