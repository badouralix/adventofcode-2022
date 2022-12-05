#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

std::vector<int> Parse(const std::string& line) {
  std::vector<int> result(4);
  int left = 0;
  std::size_t idx = 0;
  int right = 1;
  for (; right <= line.size(); ++right) {
    if (right == line.size() || line.at(right) == '-' ||
        line.at(right) == ',') {
      result.at(idx) = std::stoi(line.substr(left, right - left));
      left = right + 1;
      ++idx;
    }
  }
  return result;
}

bool IsContained(const std::vector<int>& ranges) {
  return (ranges.at(0) <= ranges.at(2) && ranges.at(3) <= ranges.at(1)) ||
         (ranges.at(2) <= ranges.at(0) && ranges.at(1) <= ranges.at(3));
}

std::string Run(const std::string& input) {
  int result = 0;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    if (IsContained(Parse(line))) {
      ++result;
    }
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
