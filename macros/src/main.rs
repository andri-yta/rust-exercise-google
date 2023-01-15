use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

struct Book {
    name: String,
    quantity: i8,
}

fn main() {
    dbg_size!(Foo);
    dbg_size!(Book);
    dbg_size!(bool);
    dbg_size!(Option<bool>);
    dbg_size!(&i32);
    dbg_size!(Option<&i32>);
}
