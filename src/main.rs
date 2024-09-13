use std::io::stdin;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err(String::from("Error: Division by zero is not allowed."))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn get_number_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_operation_input() -> String {
    let valid_operations = ["+", "-", "*", "/"];

    loop {
        let mut operation_input = String::new();

        stdin()
            .read_line(&mut operation_input)
            .expect("Failed to read operation");

        let operation_input = operation_input.trim();

        if valid_operations.contains(&operation_input) {
            return String::from(operation_input);
        } else {
            println!("Enter a valid operation (+, -, *, /): ");
        }
    }
}

fn main() {
    let first_number = get_number_input("Enter first number: ");

    println!("Enter an operation (+, -, *, /): ");
    let operation_input = get_operation_input();

    let second_number = get_number_input("Enter the second number: ");

    let op = match &operation_input[..] {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    match calculate(op) {
        Ok(result) => println!("Result: {}", result),
        Err(message) => println!("{}", message),
    }
}
