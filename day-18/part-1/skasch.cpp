#include <chrono>
#include <functional>
#include <iostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

inline constexpr int kSide = 20;
inline constexpr int kYBase = kSide;
inline constexpr int kZBase = kSide * kSide;

using Pos = int;

void VisitNeighbors(const Pos& pos, std::function<void(const Pos&)> callback) {
  callback(pos - 1);
  callback(pos + 1);
  callback(pos - kYBase);
  callback(pos + kYBase);
  callback(pos - kZBase);
  callback(pos + kZBase);
}

int CountFaces(const Pos& pos, const std::set<Pos>& blocks) {
  int count_faces = 6;
  VisitNeighbors(pos, [&](const Pos& neighbor) {
    if (blocks.contains(neighbor)) {
      count_faces -= 2;
    }
  });
  return count_faces;
}

Pos ParseLine(const std::string& line) {
  int first_comma = line.find(',');
  int x = std::stoi(line.substr(0, first_comma));
  int second_comma = line.find(',', first_comma + 1);
  int y =
      std::stoi(line.substr(first_comma + 1, second_comma - first_comma - 1));
  int z = std::stoi(line.substr(second_comma + 1));
  return x + kYBase * y + kZBase * z;
}

std::string Run(const std::string& input) {
  // Your code goes here
  int result = 0;
  std::set<Pos> blocks;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    Pos pos = ParseLine(line);
    result += CountFaces(pos, blocks);
    blocks.insert(pos);
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
