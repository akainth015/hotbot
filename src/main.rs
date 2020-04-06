use std::io;
use std::io::Write;

fn main() {
    println!("From which unit of measurement would you like to convert temperature?");
    print!("(C or F) ");
    io::stdout().flush()
        .expect("Failed to write to standard output");

    let mut input_units = String::new();

    io::stdin().read_line(&mut input_units)
        .expect("Failed to read input from standard input.");

    let input_units = input_units.trim().to_ascii_lowercase();

    let input_is_celsius = input_units == String::from("c");

    if input_is_celsius {
        println!("Converting from celsius to fahrenheit");
    } else {
        println!("Converting from fahrenheit to celsius");
    }

    let source = if input_is_celsius { "celsius" } else { "fahrenheit" };
    let target = if input_is_celsius { "fahrenheit" } else { "celsius" };

    print!("Enter the source temperature: ");
    io::stdout().flush()
        .expect("Failed to write to standard output");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input from standard input.");

    let input: f64 = input.trim().parse()
        .expect("Please enter a valid number.");

    let target_temperature = if input_is_celsius {
        celsius_to_fahrenheit(input)
    } else {
        fahrenheit_to_celsius(input)
    };

    println!("{} {} is {} {}", input, source, target_temperature, target);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}