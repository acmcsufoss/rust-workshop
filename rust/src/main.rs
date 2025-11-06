use std::env;
use std::io::{self, Write};
// use std::collections::HashMap;

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

    println!("Enter your name: ");
    print!(">> ");
    io::stdout().flush().expect("Failed to flush");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    // let color = get_color().unwrap();

    let person = &Person {
        name: String::from(line.trim()),
        color: Color::Red,
    };

    output(person)
}

// Function we will implement together :)

// fn get_color() -> Result<Color, String> {
// }

fn output(person: &Person) {
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";

    let (color_code, color_name) = match person.color {
        Color::Red => ("\x1b[38;5;196m", "RED"),
        Color::Green => ("\x1b[38;5;46m", "GREEN"),
        Color::Blue => ("\x1b[38;5;21m", "BLUE"),
    };

    const TL: &str = "╔";
    const TR: &str = "╗";
    const BL: &str = "╚";
    const BR: &str = "╝";
    const H: &str = "═";
    const V: &str = "║";

    let width = person.name.len().max(color_name.len()) + 4;
    let top_border = format!("{}{}{}", TL, H.repeat(width), TR);
    let bottom_border = format!("{}{}{}", BL, H.repeat(width), BR);

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

