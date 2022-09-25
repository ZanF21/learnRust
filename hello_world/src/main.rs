use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    println!("Hello, Pokemon!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!\nLets catch em all!");
    let width =message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
