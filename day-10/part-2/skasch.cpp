#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kScreenWidth = 40;

void PrintPixel(int cycle, int registry) {
  int pos = (cycle - 1) % kScreenWidth;
  if (registry - 1 <= pos && pos <= registry + 1) {
    std::cout << '#';
  } else {
    std::cout << '.';
  }
  if (cycle % kScreenWidth == 0) {
    std::cout << '\n';
  }
}

void Parse(const std::string& line, int& registry, int& cycle) {
  PrintPixel(cycle, registry);
  ++cycle;
  if (line.at(0) == 'n') {
    return;
  }
  PrintPixel(cycle, registry);
  registry += std::stoi(line.substr(5));
  ++cycle;
}

std::string Run(const std::string& input) {
  int registry = 1;
  int cycle = 1;
  std::cout << "_parse\n";
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    Parse(line, registry, cycle);
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
