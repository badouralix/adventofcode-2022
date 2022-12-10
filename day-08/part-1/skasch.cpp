#include <chrono>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using Matrix = std::vector<std::vector<int>>;

inline constexpr int kMaxHeight = 9;

int CharToInt(const char& c) { return c - '0'; }

void Iterate(int from, int to, std::function<int(int)> get_height,
             std::function<void(int)> add_to_visible_trees) {
  int delta = to > from ? 1 : -1;
  int max_height = -1;
  for (int idx = from; idx != to; idx += delta) {
    if (get_height(idx) > max_height) {
      add_to_visible_trees(idx);
      max_height = get_height(idx);
      if (max_height == kMaxHeight) {
        break;
      };
    }
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Matrix forest;
  for (std::string line; std::getline(iss, line);) {
    std::vector<int> row;
    for (const auto& c : line) {
      row.push_back(CharToInt(c));
    }
    forest.emplace_back(std::move(row));
  }
  std::set<std::pair<int, int>> visible_trees;
  int n_rows = forest.size();
  int n_cols = forest.at(0).size();
  for (int row = 0; row < n_rows; ++row) {
    Iterate(
        0, n_cols, [&](int col) -> int { return forest.at(row).at(col); },
        [&](int col) -> void {
          visible_trees.insert({row, col});
        });
    Iterate(
        n_cols - 1, -1, [&](int col) -> int { return forest.at(row).at(col); },
        [&](int col) -> void {
          visible_trees.insert({row, col});
        });
  }
  for (int col = 0; col < n_cols; ++col) {
    Iterate(
        0, n_rows, [&](int row) -> int { return forest.at(row).at(col); },
        [&](int row) -> void {
          visible_trees.insert({row, col});
        });
    Iterate(
        n_rows - 1, -1, [&](int row) -> int { return forest.at(row).at(col); },
        [&](int row) -> void {
          visible_trees.insert({row, col});
        });
  }
  return std::to_string(visible_trees.size());
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
