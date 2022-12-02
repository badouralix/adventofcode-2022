#include <chrono>
#include <iostream>
#include <span>
#include <string>
#include <vector>

int GetValue(const char& letter) {
  switch (letter) {
    case 'A':
    case 'X':
      return 1;
    case 'B':
    case 'Y':
      return 2;
    case 'C':
    case 'Z':
      return 3;
    default:
      exit(1);
  }
}

std::vector<std::pair<int, int>> Parse(const std::string& input) {
  std::vector<std::pair<int, int>> output{};
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    if (line.size() > 2) {
      output.emplace_back(GetValue(line.at(0)), GetValue(line.at(2)));
    }
  }
  return output;
}

int GetScore(const std::pair<int, int>& round) {
  return round.second + 3 * ((round.second - round.first + 4) % 3);
}

std::string Run(const std::string& input) {
  int result{0};
  for (const auto& round : Parse(input)) {
    result += GetScore(round);
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
