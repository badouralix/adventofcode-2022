#include <cassert>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <ostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kPivot = 0;
const std::vector<int> kKeyIndexes = {1000, 2000, 3000};
inline constexpr std::int64_t kDecryptionKey = 811589153;
inline constexpr int kRepetitions = 10;

using Values = std::vector<std::pair<std::int64_t, int>>;

std::int64_t Mod(std::int64_t a, std::int64_t b) {
  std::int64_t res = a % b;
  return res <= 0 ? res + b : res;
}
std::ostream& operator<<(std::ostream& out, const Values& values) {
  out << "[ ";
  for (const auto& [value, _] : values) {
    out << value << ' ';
  }
  out << "]\n";
  return out;
}

void ApplyMoves(Values& values) {
  for (int value_index = 0; value_index < values.size(); ++value_index) {
    int index = 0;
    for (; index < values.size(); ++index) {
      if (values.at(index).second == value_index) {
        break;
      }
    }
    std::int64_t value = values.at(index).first;
    values.erase(values.begin() + index);
    int new_index = Mod(static_cast<std::int64_t>(index) + value,
                        static_cast<std::int64_t>(values.size()));
    values.insert(values.begin() + new_index, {value, value_index});
  }
}

std::int64_t Decrypt(const Values& values) {
  int pos_pivot = 0;
  for (int index = 0; index < values.size(); ++index) {
    if (values.at(index).first == kPivot) {
      pos_pivot = index;
      break;
    }
  }
  std::int64_t result = 0;
  for (int key_index : kKeyIndexes) {
    result += values.at((pos_pivot + key_index) % values.size()).first;
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  Values values;
  std::istringstream iss(input);
  std::string line;
  for (int index = 0; std::getline(iss, line); ++index) {
    int number = std::stoi(line);
    values.push_back(
        {kDecryptionKey * static_cast<std::int64_t>(number), index});
  }
  for (int repetition = 0; repetition < kRepetitions; ++repetition) {
    ApplyMoves(values);
  }
  return std::to_string(Decrypt(values));
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
