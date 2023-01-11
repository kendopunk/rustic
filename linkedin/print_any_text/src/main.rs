/**
 * Print Any Text
 * info()
 * String or string slice as an argumentinfo()
 * print to stdout
 */
use std::fmt::Display;
fn main() {
    let s = String::from("How now brown cow?");
    info(&s);

    let s = "How now brown cow?";
    info(&s);

    let jack = 1.224;
    info(&jack);
}

fn info<T: Display>(text: &T) {
    println!("info: {}", text);
}
