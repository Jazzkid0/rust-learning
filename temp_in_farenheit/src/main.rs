use std::io;

fn main() {
    println!("Please input a temperature in celcius to convert to farenheit:")

    let mut input = String new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.")
}
