use std::io;

fn main() {
    println!("Please input a temperature in celcius to convert to farenheit:");

    
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("{}F", input * (5.0/9.0) + 32.0);
        break;
    }

        
}
