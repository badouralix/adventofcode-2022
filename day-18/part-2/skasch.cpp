#include <chrono>
#include <functional>
#include <iostream>
#include <ostream>
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

std::tuple<int, int, int> GetXYZ(const Pos& pos) {
  int x = pos % kSide;
  int y = (pos / kSide) % kSide;
  int z = pos / kZBase;
  return {x, y, z};
}

void Display(std::ostream& out, const Pos& pos) {
  auto [x, y, z] = GetXYZ(pos);
  out << '(' << x << ',' << y << ',' << z << ')';
}

void Display(std::ostream& out, const std::set<Pos>& blocks) {
  for (int x = 0; x < kSide; ++x) {
    out << "Slice x = " << x << '\n';
    for (int y = 0; y < kSide; ++y) {
      for (int z = 0; z < kSide; ++z) {
        Pos pos = x + kYBase * y + kZBase * z;
        if (blocks.contains(pos)) {
          out << '#';
        } else {
          out << ' ';
        }
      }
      out << '\n';
    }
  }
}

void VisitNeighbors(const Pos& pos, std::function<void(const Pos&)> callback) {
  auto [x, y, z] = GetXYZ(pos);
  if (x > 0) {
    callback(pos - 1);
  }
  if (x < kSide - 1) {
    callback(pos + 1);
  }
  if (y > 0) {
    callback(pos - kYBase);
  }
  if (y < kSide - 1) {
    callback(pos + kYBase);
  }
  if (z > 0) {
    callback(pos - kZBase);
  }
  if (z < kSide - 1) {
    callback(pos + kZBase);
  }
}

std::set<Pos> Dfs(std::set<Pos> blocks) {
  std::vector<Pos> stack = {0};
  std::set<Pos> remaining;
  for (Pos pos = 0; pos < kZBase * kSide; ++pos) {
    remaining.insert(pos);
  }
  while (!stack.empty()) {
    Pos pos = stack.back();
    stack.pop_back();
    if (!remaining.contains(pos)) {
      continue;
    }
    remaining.erase(pos);
    VisitNeighbors(pos, [&](const Pos& neighbor) {
      if (!remaining.contains(neighbor)) {
        return;
      }
      if (blocks.contains(neighbor)) {
        return;
      }
      stack.emplace_back(neighbor);
    });
  }
  return remaining;
}

int CountFaces(const Pos& pos, const std::set<Pos>& blocks) {
  int count_faces = 6;
  VisitNeighbors(pos, [&](const Pos& neighbor) {
    if (blocks.contains(neighbor)) {
      count_faces -= 1;
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
  std::set<Pos> blocks;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    Pos pos = ParseLine(line);
    blocks.insert(pos);
  }
  std::set<Pos> closure_blocks = Dfs(blocks);
  int result = 0;
  for (const Pos& pos : closure_blocks) {
    result += CountFaces(pos, closure_blocks);
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
