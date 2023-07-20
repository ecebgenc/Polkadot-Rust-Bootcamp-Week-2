use std::io::{self, Write};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, &'static str> {
     match operation {
        Operation::Add(x, y) => Ok(x + y),
        Operation::Subtract(x, y) => Ok(x - y),
        Operation::Multiply(x, y) => Ok(x * y),
        Operation::Divide(x, y) => {
            if y != 0.0 {
                Ok(x / y)
            } else {
                Err("Zero division error!")
            }
        }
     }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    std::io::stdin().read_line(&mut buff).unwrap();
    buff.trim().to_string()
}
fn main() {
    let first_number_str = get_input("Enter the first number:");
    let first_number: f64 = first_number_str.parse().expect("Invalid input");

    let operation_symbol = get_input("Enter the operation (+, -, *, /):");

    let second_number_str = get_input("Enter the second number:");
    let second_number: f64 = second_number_str.parse().expect("Invalid input");

    let operation = match operation_symbol.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation symbol. Please use one of +, -, *, /.");
            return;
        }
    };
    
    match calculate(operation) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

