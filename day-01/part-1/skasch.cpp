#include <algorithm>
#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

std::vector<int> Parse(const std::string& input) {
  std::vector<int> output{};
  int acc{0};
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    if (line.empty()) {
      output.push_back(acc);
      acc = 0;
    } else {
      acc += std::stoi(line);
    }
  }
  output.push_back(acc);
  return output;
}

std::string Run(const std::string& input) {
  std::vector<int> caloriesList{Parse(input)};
  const int answer{*std::max_element(caloriesList.begin(), caloriesList.end())};
  return std::to_string(answer);
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
