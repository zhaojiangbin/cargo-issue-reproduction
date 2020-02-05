use three::{Three};
use std::env;
use std::io::{stdout};

fn main() {
    let arg = env::args().nth(1).expect("need an arg");
    arg.as_bytes().write(&mut stdout()).unwrap();
}
