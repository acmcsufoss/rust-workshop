#include <algorithm>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_map>

enum Color {
  RED,
  BLUE,
  GREEN,
  YELLOW,
  PINK,
  PURPLE,
  ORANGE,
};

struct Person {
  std::string name;
  Color color;
};

/**
 * Helper function that reads and validates color input from user
 * @throws invalid_argument if color that's read isn't valid
 */
Color get_color();

/**
 * Helper function to pretty-print the output
 */
void output(const Person &person) noexcept;

// Main Point of Entry! =======================================================
int main(int argc, char **argv) {
  if (argc > 1) {
    std::cerr << "Error: " << argv[0] << " expects no arguments\n";
    exit(1);
  }

  /*
   * Normally you don't want to use `new` and `delete` at all (you should be
   * using smart pointers).
   * However we're trying to illustrate a point here so
   */
  Person *person = new Person;

  std::cout << "Enter your name: \n";
  std::cout << ">> ";
  std::getline(std::cin, person->name);

  Color color;
  try {
    person->color = get_color();
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << "\n";
    exit(1);
  }

  /*
   * BUG HERE!!
   * This is a 'use after free' error, which is something that no C++ compiler
   * will catch and warn you about. The rust compiler will, though.
   */
  delete person;
  output(*person);

  return 0;
}

// This is the function we're going to re-implement in Rust!
Color get_color() {
  // Read input
  std::cout << "Enter your favorite color (one of RED, BLUE, GREEN, YELLOW, PINK, PURPLE, ORANGE): \n";
  std::cout << ">> ";
  std::string color_str;
  std::getline(std::cin, color_str);

  // Validate input
  std::transform(color_str.begin(), color_str.end(), color_str.begin(),
                 ::tolower);
  static const std::unordered_map<std::string, Color> str_enum_map = {
      {"red", RED},       {"blue", BLUE}, {"green", GREEN},
      {"yellow", YELLOW}, {"pink", PINK}, {"purple", PURPLE},
      {"orange", ORANGE},
  };
  auto it = str_enum_map.find(color_str);
  if (it == str_enum_map.end()) {
    throw std::invalid_argument("invalid color or color not supported");
  }

  return str_enum_map.at(color_str);
}

// ================= Not super important =================
void output(const Person &person) noexcept {
  // ANSI color codes
  const std::string RESET = "\033[0m";
  const std::string BOLD = "\033[1m";

  std::string color_code;
  std::string color_name;

  // To print terminal colors
  switch (person.color) {
  case Color::RED:
    color_code = "\033[38;5;196m"; // Bright red
    color_name = "RED";
    break;
  case Color::GREEN:
    color_code = "\033[38;5;46m"; // Bright green
    color_name = "GREEN";
    break;
  case Color::BLUE:
    color_code = "\033[38;5;21m"; // Bright blue
    color_name = "BLUE";
    break;
  case Color::YELLOW:
    color_code = "\033[38;5;226m"; // Bright yellow
    color_name = "YELLOW";
    break;
  case Color::PINK:
    color_code = "\033[38;5;213m"; // Pink
    color_name = "PINK";
    break;
  case Color::PURPLE:
    color_code = "\033[38;5;129m"; // Purple
    color_name = "PURPLE";
    break;
  case Color::ORANGE:
    color_code = "\033[38;5;208m"; // Orange
    color_name = "ORANGE";
    break;
  }

  const std::string TL = "╔";
  const std::string TR = "╗";
  const std::string BL = "╚";
  const std::string BR = "╝";
  const std::string H = "=";
  const std::string V = "║";

  int width = std::max(person.name.length(), color_name.length()) + 4;
  std::string top_border = TL + std::string(width, H[0]) + TR;
  std::string bottom_border = BL + std::string(width, H[0]) + BR;

  std::cout << top_border << "\n";
  std::cout << V << " " << BOLD << std::setw(width - 1) << std::left
            << person.name << RESET << V << "\n";
  std::cout << V << " " << std::setw(width - 1) << std::left << "Color:" << V
            << "\n";
  std::cout << V << " " << BOLD << color_code << std::setw(width - 1)
            << std::left << color_name << RESET << V << "\n";
  std::cout << bottom_border << "\n";
}
