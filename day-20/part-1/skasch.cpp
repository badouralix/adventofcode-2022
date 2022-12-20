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

using Values = std::vector<std::pair<int, bool>>;

int Mod(int a, int b) {
  int res = a % b;
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
  int index = 0;
  for (int op = 0; op < values.size(); ++op) {
    while (values.at(index).second) {
      ++index;
    }
    int value = values.at(index).first;
    values.erase(values.begin() + index);
    int new_index = Mod(index + value, values.size());
    values.insert(values.begin() + new_index, {value, true});
  }
}

int Decrypt(const Values& values) {
  int pos_pivot = 0;
  for (int index = 0; index < values.size(); ++index) {
    if (values.at(index).first == kPivot) {
      pos_pivot = index;
      break;
    }
  }
  int result = 0;
  for (int key_index : kKeyIndexes) {
    result += values.at((pos_pivot + key_index) % values.size()).first;
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  Values values;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    int number = std::stoi(line);
    values.push_back({number, false});
  }
  ApplyMoves(values);
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
