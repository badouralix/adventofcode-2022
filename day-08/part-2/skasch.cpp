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
using Stack = std::vector<std::pair<int, int>>;

int CharToInt(const char& c) { return c - '0'; }

void Iterate(int from, int to, std::function<int(int)> get_height,
             std::function<void(int)> update_score_when_empty,
             std::function<void(int, int)> update_score_with_prev_index,
             std::function<void(int)> update_best_score) {
  int delta = to > from ? 1 : -1;
  Stack stack{};
  for (int idx = from; idx != to; idx += delta) {
    int height = get_height(idx);
    while (!stack.empty() && height > stack.back().first) {
      stack.pop_back();
    }
    if (stack.empty()) {
      update_score_when_empty(idx);
    } else {
      int prev_idx = stack.back().second;
      if (stack.back().first == height) {
        stack.pop_back();
      }
      update_score_with_prev_index(idx, prev_idx);
    }
    stack.emplace_back(height, idx);
    update_best_score(idx);
  }
}

int& GetAt(Matrix& matrix, int row, int col) { return matrix.at(row).at(col); }

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
  int n_rows = forest.size();
  int n_cols = forest.at(0).size();
  Matrix scores;
  for (int row = 0; row < n_rows; ++row) {
    scores.emplace_back(n_cols, 1);
  }
  int best_score = 1;
  for (int row = 0; row < n_rows; ++row) {
    Iterate(
        0, n_cols, [&](int col) { return forest.at(row).at(col); },
        [&](int col) { scores.at(row).at(col) *= col; },
        [&](int col, int prev_col) {
          scores.at(row).at(col) *= (col - prev_col);
        },
        [&](int) {});
    Iterate(
        n_cols - 1, -1, [&](int col) { return forest.at(row).at(col); },
        [&](int col) { scores.at(row).at(col) *= n_cols - col - 1; },
        [&](int col, int next_col) {
          scores.at(row).at(col) *= (next_col - col);
        },
        [&](int) {});
  }
  for (int col = 0; col < n_cols; ++col) {
    Iterate(
        0, n_rows, [&](int row) { return forest.at(row).at(col); },
        [&](int row) { scores.at(row).at(col) *= row; },
        [&](int row, int prev_row) {
          scores.at(row).at(col) *= (row - prev_row);
        },
        [&](int) {});
    Iterate(
        n_rows - 1, -1, [&](int row) { return forest.at(row).at(col); },
        [&](int row) { scores.at(row).at(col) *= n_rows - row - 1; },
        [&](int row, int next_row) {
          scores.at(row).at(col) *= (next_row - row);
        },
        [&](int row) {
          if (scores.at(row).at(col) > best_score) {
            best_score = scores.at(row).at(col);
          }
        });
  }
  return std::to_string(best_score);
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
