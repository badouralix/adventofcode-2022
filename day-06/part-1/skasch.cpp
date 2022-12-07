#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kLetters = 26;
inline constexpr int packetSize = 4;

std::string Run(const std::string& input) {
  std::vector<int> character_counts(kLetters);
  int n_uniques = 0;
  for (int idx = 0; idx < packetSize; ++idx) {
    ++character_counts.at(input.at(idx) - 'a');
    if (character_counts.at(input.at(idx) - 'a') == 1) {
      ++n_uniques;
    }
  }
  for (int idx = packetSize; idx < input.size(); ++idx) {
    if (n_uniques == packetSize) {
      return std::to_string(idx);
    }
    --character_counts.at(input.at(idx - packetSize) - 'a');
    if (character_counts.at(input.at(idx - packetSize) - 'a') == 0) {
      --n_uniques;
    }
    ++character_counts.at(input.at(idx) - 'a');
    if (character_counts.at(input.at(idx) - 'a') == 1) {
      ++n_uniques;
    }
  }
  return std::to_string(input.size());
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
