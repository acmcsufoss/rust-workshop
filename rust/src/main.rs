use std::env;
use std::io::{self, Write};
use std::collections::HashMap;

// Simple data structure representing a person with a name and a favorite color
struct Person {
    name: String,
    color: Color,
}


// Enum for supported colors. Derive Clone so we can copy an owned Color out of collections
#[derive(Clone)]
enum Color {
    Red,
    Blue,
    Green,
}


fn main() {
    // Validate command-line usage: this program expects no arguments
    // If any arguments are present, print an error message and exit with a non-zero code
    if env::args().len() > 1 {
        eprintln!(
            "Error: {} exptects no arguments",
            env::args().next().unwrap()
        );
        std::process::exit(1);
    }

    // Prompt the user for their name. We flush stdout after printing the prompt to ensure the prompt appears before read_line blocks for input
    println!("Enter your name: ");
    print!(">> ");
    io::stdout().flush().expect("Failed to flush");

    // Read the name line into a String, trimming trailing newline later when constructing Person
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    // Ask for the favorite color and convert the result to a Color
    // get_color returns a Result<Color, String>; unwrap() is used here which will panic and terminate the program if the user input is invalid
    let color = get_color().unwrap();

    // Build a Person instance from the collected name and color. Trim whitespace from the name
    let person = &Person {
        name: String::from(line.trim()),
        color: color,
    };

    // Note: dropping the reference `person` explicitly would be unnecessary and could cause confusion; ownership and borrow rules manage lifetimes automatically in Rust
    // drop(person);

    // Print a formatted output box for the person
    output(person)
}


// Function we will implement together :)
// Function to prompt and parse a color string into the Color enum
// Returns Result<Color, String> so caller can decide how to handle invalid input
fn get_color() -> Result<Color, String> {
    // Prompt the user for a color choice
    println!("Enter your favorite color (one of RED, BLUE, GREEN):");
    let mut color_str = String::new();
    io::stdin()
        .read_line(&mut color_str)
        .expect("Failed to read line");

    // Normalize user input: trim whitespace and convert to lowercase for comparison
    let color_str = color_str.trim().to_lowercase();

    // Map string literals to Color variants. Using HashMap lets us lookup by string
    // The HashMap holds owned Color values; we clone the Color when returning to give the caller an owned value
    let str_enum_map = HashMap::from([
        ("red", Color::Red),
        ("blue", Color::Blue),
        ("green", Color::Green),
    ]);

    // Lookup the normalized string in the map. If found, clone and return Ok(Color)
    // If not found, return an Err with a descriptive message
    // This expression is the function's return value (no trailing semicolon)
    str_enum_map
        .get(color_str.as_str())
        .cloned()
        .ok_or_else(|| String::from("invalid color"))
}


// Pretty-print the Person using ANSI color codes and box drawing characters
// Takes a reference to Person so we don't transfer ownership
fn output(person: &Person) {
    // ANSI reset and bold codes used to style the output
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";

    // Choose an ANSI color code and a display name based on the Person's Color
    // Matching on the enum borrows the enum via pattern matching
    let (color_code, color_name) = match person.color {
        Color::Red => ("\x1b[38;5;196m", "RED"),
        Color::Green => ("\x1b[38;5;46m", "GREEN"),
        Color::Blue => ("\x1b[38;5;21m", "BLUE"),
    };

    // Box drawing characters for a simple framed output
    const TL: &str = "╔";
    const TR: &str = "╗";
    const BL: &str = "╚";
    const BR: &str = "╝";
    const H: &str = "═";
    const V: &str = "║";

    // Compute the width of the inner area so the box fits the longest line (name or color)
    let width = person.name.len().max(color_name.len()) + 4;
    let top_border = format!("{}{}{}", TL, H.repeat(width), TR);
    let bottom_border = format!("{}{}{}", BL, H.repeat(width), BR);

    // Print the top border and the boxed content lines with proper padding
    println!("{}", top_border);
    println!(
        "{} {}{:<width$}{}{}",
        V,
        BOLD,
        person.name,
        RESET,
        V,
        width = width - 1
    );
    println!("{} {:<width$}{}", V, "Color:", V, width = width - 1);
    println!(
        "{} {}{}{:<width$}{}{}",
        V,
        BOLD,
        color_code,
        color_name,
        RESET,
        V,
        width = width - 1
    );
    println!("{}", bottom_border);
}