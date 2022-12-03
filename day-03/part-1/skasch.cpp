#include <algorithm>
#include <chrono>
#include <iostream>
#include <ranges>
#include <span>
#include <sstream>
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

std::vector<std::pair<ItemSet, ItemSet>> Parse(const std::string& input) {
  std::vector<std::pair<ItemSet, ItemSet>> output{};
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    ItemSet compartment1{};
    ItemSet compartment2{};
    const int compartmentSize{static_cast<int>(line.length()) / 2};
    for (int idx{0}; idx < compartmentSize; ++idx) {
      compartment1.insert(line.at(idx));
      compartment2.insert(line.at(idx + compartmentSize));
    }
    output.emplace_back(compartment1, compartment2);
  }
  return output;
}

std::string Run(const std::string& input) {
  std::vector<std::pair<ItemSet, ItemSet>> rucksacks{Parse(input)};
  int result{};
  for (const auto& [compartment1, compartment2] : rucksacks) {
    for (const auto& item : compartment1) {
      if (compartment2.contains(item)) {
        result += GetScore(item);
        continue;
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
