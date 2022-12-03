#include <algorithm>
#include <chrono>
#include <iostream>
#include <ranges>
#include <span>
#include <string>
#include <unordered_set>
#include <vector>

using ItemSet = std::unordered_set<char>;

int GetScore(char item) {
  if (item >= 'a') {
    return static_cast<int>(item) - static_cast<int>('a') + 1;
  }
  return static_cast<int>(item) - static_cast<int>('A') + 27;
}

std::vector<ItemSet> Parse(const std::string& input) {
  std::vector<ItemSet> output{};
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    output.emplace_back(line.begin(), line.end());
  }
  return output;
}

std::string Run(const std::string& input) {
  std::vector<ItemSet> rucksacks{Parse(input)};
  int result{};
  for (int idx{0}; idx < rucksacks.size(); idx += 3) {
    for (const auto& item : rucksacks.at(idx)) {
      if (rucksacks.at(idx + 1).contains(item) &&
          rucksacks.at(idx + 2).contains(item)) {
        result += GetScore(item);
      }
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
