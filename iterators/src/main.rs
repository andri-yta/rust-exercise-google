fn main() {
    // iter
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");

    // into_iter
    let w: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = w.into_iter();

    let w0: Option<String> = iter.next();
    println!("w0: {w0:?}");

    // loops
    let x: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // loop implementing into_iter for &Vec<T>
    for word in &x {
        println!("word: {word}");
    }

    // loop implementing into_iter for Vec<T>
    // loop takes ownership
    for word in x {         
        println!("word: {word}");
    }

}