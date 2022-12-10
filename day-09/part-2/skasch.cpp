#include <chrono>
#include <iostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

inline constexpr int kRopeSize = 10;

using Pos = std::pair<int, int>;
struct Move {
  char direction;
  int steps;
};

int& Row(Pos& pos) { return pos.first; }
int& Col(Pos& pos) { return pos.second; }
int Row(const Pos& pos) { return pos.first; }
int Col(const Pos& pos) { return pos.second; }

Move ParseLine(const std::string& line) {
  return {line.at(0), std::stoi(line.substr(2))};
}

void Step(const char& direction, Pos& head) {
  switch (direction) {
    case 'U':
      --Row(head);
      break;
    case 'D':
      ++Row(head);
      break;
    case 'R':
      ++Col(head);
      break;
    case 'L':
      --Col(head);
      break;
    default:
      exit(1);
  }
}

void UpdateTail(const Pos& head, Pos& tail) {
  if (Row(head) - 1 <= Row(tail) && Row(tail) <= Row(head) + 1 &&
      Col(head) - 1 <= Col(tail) && Col(tail) <= Col(head) + 1) {
    return;
  }
  if (Row(head) - 1 <= Row(tail) && Row(tail) <= Row(head) + 1) {
    Row(tail) = Row(head);
    if (Col(head) < Col(tail)) {
      --Col(tail);
    } else {
      ++Col(tail);
    }
    return;
  }
  if (Col(head) - 1 <= Col(tail) && Col(tail) <= Col(head) + 1) {
    Col(tail) = Col(head);
    if (Row(head) < Row(tail)) {
      --Row(tail);
    } else {
      ++Row(tail);
    }
    return;
  }
  if (Col(tail) < Col(head)) {
    ++Col(tail);
  } else {
    --Col(tail);
  }
  if (Row(tail) < Row(head)) {
    ++Row(tail);
  } else {
    --Row(tail);
  }
}

std::string Run(const std::string& input) {
  std::istringstream iss(input);
  std::vector<Pos> rope(kRopeSize, {0, 0});
  std::set<Pos> visited;
  visited.insert(rope.back());
  for (std::string line; std::getline(iss, line);) {
    Move move = ParseLine(line);
    for (int step = 0; step < move.steps; ++step) {
      Step(move.direction, rope.front());
      for (int tail_index = 1; tail_index < kRopeSize; ++tail_index) {
        UpdateTail(rope.at(tail_index - 1), rope.at(tail_index));
      }
      visited.insert(rope.back());
    }
  }
  return std::to_string(visited.size());
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
