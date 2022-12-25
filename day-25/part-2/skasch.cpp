#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
  }
  return "";
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    std::cout << "Missing one argument" << std::endl;
    exit(1);
  }
  auto args = std::span(argv, static_cast<size_t>(argc));

  auto start = std::chrono::high_resolution_clock::now();
  auto answer = Run(args[1]);
  auto end = std::chrono::high_resolution_clock::now();

  std::cout << "_duration:"
            << std::chrono::duration<float, std::milli>(end - start).count()
            << "\n";

  std::cout << answer << "\n";
  return 0;
}
