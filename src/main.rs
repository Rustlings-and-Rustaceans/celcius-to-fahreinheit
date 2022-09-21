use std::io;

fn main() {
    println!("**\t Convert Celsius to Fahrenheit\t **");

    // Prompt User for input
    println!("Enter the celsius to convert: ");

    // Store celsius value input
    let mut celsius_value = String::new();

    // Capture value entered from the terminal
    io::stdin().read_line(&mut celsius_value).expect("There was an error reading the value");

    // Convert value captured to 32 bit float value
    let celsius_value: f32 = celsius_value.trim().parse().expect("Please enter a number");

    // Convert value to fahrenheit
    let fahrenheit_value = (celsius_value * 9.0 / 5.0) + 32.0;

    // Display converted value to user
    let deg_symbol = '\u{00B0}';
    println!("{}{}C Celsius converted to Fahrenheit is {:.2}{}F", celsius_value, deg_symbol, fahrenheit_value, deg_symbol);
}
