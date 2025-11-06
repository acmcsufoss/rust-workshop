use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

struct Person {
    name: String,
    color: Color,
}

#[derive(Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    if env::args().len() > 1 {
        eprintln!(
            "Error: {} exptects no arguments",
            env::args().next().unwrap()
        );
        std::process::exit(1);
    }

    // Read name
    println!("Enter your name: ");
    print!(">> ");
    io::stdout().flush().expect("Failed to flush");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    // This will panic and exit the program if the error condition is returned
    let color = get_color().unwrap();

    let person = &Person {
        name: String::from(line.trim()),
        color: color,
    };

    /*
     * This `drop` call is kinda like `delete` C++. Try uncommenting this and see if the program
     * compiles with the same bug our C++ code had.
     */
    // drop(person);
    output(person)
}

// Done!!
fn get_color() -> Result<Color, String> {
    println!("Enter your favorite color (one of RED, BLUE, GREEN):");
    let mut color_str = String::new();
    io::stdin()
        .read_line(&mut color_str)
        .expect("Failed to read line");

    let color_str = color_str.trim().to_lowercase();

    let str_enum_map = HashMap::from([
        ("red", Color::Red),
        ("blue", Color::Blue),
        ("green", Color::Green),
    ]);

    // No return keyword - function returns last expression (dont use a semicolon though)
    str_enum_map
        .get(color_str.as_str())
        .cloned()
        .ok_or_else(|| String::from("invalid color"))
}

// You can ignore this function ===================================================================
fn output(person: &Person) {
    // ANSI color codes
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";

    let (color_code, color_name) = match person.color {
        Color::Red => ("\x1b[38;5;196m", "RED"),
        Color::Green => ("\x1b[38;5;46m", "GREEN"),
        Color::Blue => ("\x1b[38;5;21m", "BLUE"),
    };

    // Box drawing characters
    const TL: &str = "╔";
    const TR: &str = "╗";
    const BL: &str = "╚";
    const BR: &str = "╝";
    const H: &str = "═";
    const V: &str = "║";

    let width = person.name.len().max(color_name.len()) + 4;
    let top_border = format!("{}{}{}", TL, H.repeat(width), TR);
    let bottom_border = format!("{}{}{}", BL, H.repeat(width), BR);

    // Print the fancy table
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
