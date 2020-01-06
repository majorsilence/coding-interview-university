use std::io;

fn main() {
    
    
    loop {

        println!("Enter F temp to convert to C temp:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp = (input - 32.0) * 5.0/9.0;
        println!("{} F is {} C", input, temp);
        break;
    }
}
