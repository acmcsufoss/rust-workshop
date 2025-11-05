#include <algorithm>
// #include <bits/stdc++.h>
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

Color get_color();
void output(const Person &person);

int main(int argc, char **argv) {
  if (argc > 1) {
    std::cerr << "Error: " << argv[0] << " expects no arguments\n";
    exit(1);
  }

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

  output(*person);

  delete person;

  return 0;
}

Color get_color() {
  // Read input
  std::cout << "Enter your favorite color (one of RED, BLUE, GREEN, YELLOW, "
               "PINK, PURPLE, ORANGE): \n";
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

  // Input is valid
  return str_enum_map.at(color_str);
}

void output(const Person &person) {
  // ANSI color codes
  const std::string RESET = "\033[0m";
  const std::string BOLD = "\033[1m";

  std::string color_code;
  std::string color_name;

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

  // Box drawing characters
  const std::string TL = "╔";
  const std::string TR = "╗";
  const std::string BL = "╚";
  const std::string BR = "╝";
  const std::string H = "═";
  const std::string V = "║";

  int width = std::max(person.name.length(), color_name.length()) + 4;
  std::string top_border = TL + std::string(width, H[0]) + TR;
  std::string bottom_border = BL + std::string(width, H[0]) + BR;

  // Print the fancy table
  std::cout << top_border << "\n";
  std::cout << V << " " << BOLD << std::setw(width - 1) << std::left
            << person.name << RESET << V << "\n";
  std::cout << V << " " << std::setw(width - 1) << std::left << "Color:" << V
            << "\n";
  std::cout << V << " " << BOLD << color_code << std::setw(width - 1)
            << std::left << color_name << RESET << V << "\n";
  std::cout << bottom_border << "\n";
}
