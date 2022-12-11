#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

static const std::vector<int> kCycles = {20, 60, 100, 140, 180, 220};

int Parse(const std::string& line, int registry, int& cycle) {
  if (line.at(0) == 'n') {
    ++cycle;
    return registry;
  }
  cycle += 2;
  return registry + std::stoi(line.substr(5));
}

std::string Run(const std::string& input) {
  int result = 0;
  int cycle_index = 0;
  int registry = 1;
  int cycle = 1;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    int next_registry = Parse(line, registry, cycle);
    if (cycle > kCycles.at(cycle_index)) {
      result += registry * kCycles.at(cycle_index);
      ++cycle_index;
    } else if (cycle == kCycles.at(cycle_index)) {
      result += next_registry * kCycles.at(cycle_index);
      ++cycle_index;
    }
    if (cycle_index >= kCycles.size()) {
      break;
    }
    registry = next_registry;
  }
  return std::to_string(result);
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
