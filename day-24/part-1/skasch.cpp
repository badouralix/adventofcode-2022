#include <cassert>
#include <chrono>
#include <functional>
#include <iostream>
#include <map>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <vector>

using Pos = std::pair<int, int>;

int Row(const Pos& pos) { return pos.first; }
int Col(const Pos& pos) { return pos.second; }

Pos operator+(const Pos& lhs, const Pos& rhs) {
  return {Row(lhs) + Row(rhs), Col(lhs) + Col(rhs)};
}

std::ostream& operator<<(std::ostream& out, const Pos& pos) {
  out << '(' << Row(pos) << ',' << Col(pos) << ')';
  return out;
}

static const std::unordered_map<char, Pos> kDirectionDeltas{
    {'^', {-1, 0}},
    {'>', {0, 1}},
    {'v', {1, 0}},
    {'<', {0, -1}},
};

struct Map {
  int n_inner_rows;
  int n_inner_cols;
  std::multimap<Pos, Pos> blizzards;
};

Pos Wrap(const Pos& pos, const Map& map) {
  if (Row(pos) == 0) {
    return {map.n_inner_rows, Col(pos)};
  } else if (Row(pos) == map.n_inner_rows + 1) {
    return {1, Col(pos)};
  } else if (Col(pos) == 0) {
    return {Row(pos), map.n_inner_cols};
  } else if (Col(pos) == map.n_inner_cols + 1) {
    return {Row(pos), 1};
  }
  return pos;
}

bool IsValid(const Pos& pos, const Map& map) {
  return 1 <= Row(pos) && Row(pos) <= map.n_inner_rows && 1 <= Col(pos) &&
         Col(pos) <= map.n_inner_cols;
}

void MoveMap(Map& map) {
  std::multimap<Pos, Pos> blizzards;
  for (const auto& [pos, delta] : map.blizzards) {
    Pos next_pos = Wrap(pos + delta, map);
    blizzards.insert({next_pos, delta});
  }
  std::swap(blizzards, map.blizzards);
}

void Move(const Pos& pos, const Map& map,
          std::function<void(const Pos&)> callback) {
  if (Row(pos) == 0) {
    callback(pos);
    Pos delta = kDirectionDeltas.at('v');
    if (!map.blizzards.contains(pos + delta)) {
      callback(pos + delta);
    }
    return;
  }
  if (!map.blizzards.contains(pos)) {
    callback(pos);
  }
  for (const auto& [_, delta] : kDirectionDeltas) {
    if (IsValid(pos + delta, map) && !map.blizzards.contains(pos + delta)) {
      callback(pos + delta);
    }
  }
}

struct Input {
  Map map;
  int start_col;
  int end_col;
};

int Bfs(Map& map, Pos start, Pos target) {
  std::set<Pos> positions = {start};
  std::set<Pos> next_positions;
  int steps = 0;
  while (true) {
    ++steps;
    MoveMap(map);
    for (const Pos& position : positions) {
      Move(position, map, [&](const Pos& pos) { next_positions.insert(pos); });
      if (next_positions.contains(target)) {
        MoveMap(map);
        return steps + 1;
      }
    }
    std::swap(positions, next_positions);
    next_positions.clear();
  }
}

Input Parse(const std::string& input) {
  std::multimap<Pos, Pos> blizzards;
  int n_inner_cols = -1;
  int start = -1;
  int end = -1;
  int row = 0;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line); ++row) {
    if (start == -1) {
      start = line.find('.');
      n_inner_cols = line.size() - 2;
      continue;
    }
    if (line.at(1) == '#' || line.at(2) == '#') {
      assert(end == -1);
      end = line.find('.');
      continue;
    }
    int col = 0;
    for (const auto& c : line) {
      if (kDirectionDeltas.contains(c)) {
        blizzards.insert({{row, col}, kDirectionDeltas.at(c)});
      }
      ++col;
    }
  }
  return {
      {
          row - 2,
          n_inner_cols,
          blizzards,
      },
      start,
      end,
  };
}

std::string Run(const std::string& input) {
  // Your code goes here
  Input parsed_input = Parse(input);
  return std::to_string(
      Bfs(parsed_input.map, {0, parsed_input.start_col},
          {parsed_input.map.n_inner_rows, parsed_input.end_col}));
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
