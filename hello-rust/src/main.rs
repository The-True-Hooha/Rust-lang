use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("welcome to my first rust-program");
    println!("meet crab, the official rust mascot");

    let stdout = stdout();
    let message = String::from("hello, check it out below");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
