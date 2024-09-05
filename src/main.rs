use std::io::{self, Write};
use whatlang::{detect, Lang, Script};

// Creating a function to detect language
fn what_language(text: &str) {
    let info = detect(text).unwrap();
    println!("Language type: {:?}", info.lang());
    println!("Language Script: {:?}", info.script());
    println!("System confidence: {}", info.confidence());
    println!("System reliability: {}", info.is_reliable());

    if info.is_reliable() {
        println!("The detected language is reliable: True");
    } else {
        println!("The detected language is reliable: False");
    }
}

fn main() {
    // Prompt the user for input
    print!("Enter some text: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the trailing newline character
    let input = input.trim();

    // Call the function with user input
    what_language(input);
}