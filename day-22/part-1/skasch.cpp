#include <algorithm>
#include <chrono>
#include <iostream>
#include <map>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <variant>

inline constexpr int kRowFactor = 1000;
inline constexpr int kColFactor = 4;

using Pos3 = std::pair<int, int>;

struct Map {
  std::map<int, int> left_bounds;
  std::map<int, int> right_bounds;
  std::map<int, int> top_bounds;
  std::map<int, int> bottom_bounds;
  std::set<Pos3> walls;
};

enum class Rotation {
  Left,
  Right,
};

enum class Orientation {
  Right,
  Bottom,
  Left,
  Top,
};

static Orientation PreviousOrientation(const Orientation& orientation) {
  switch (orientation) {
    case Orientation::Right:
      return Orientation::Top;
    case Orientation::Bottom:
      return Orientation::Right;
    case Orientation::Left:
      return Orientation::Bottom;
    case Orientation::Top:
      return Orientation::Left;
  }
  exit(1);
}

static Orientation NextOrientation(const Orientation& orientation) {
  switch (orientation) {
    case Orientation::Right:
      return Orientation::Bottom;
    case Orientation::Bottom:
      return Orientation::Left;
    case Orientation::Left:
      return Orientation::Top;
    case Orientation::Top:
      return Orientation::Right;
  }
  exit(1);
}

static Pos3 ToDelta(const Orientation& orientation) {
  switch (orientation) {
    case Orientation::Right:
      return {0, 1};
    case Orientation::Bottom:
      return {1, 0};
    case Orientation::Left:
      return {0, -1};
    case Orientation::Top:
      return {-1, 0};
  }
  exit(1);
}

static int ToValue(const Orientation& orientation) {
  switch (orientation) {
    case Orientation::Right:
      return 0;
    case Orientation::Bottom:
      return 1;
    case Orientation::Left:
      return 2;
    case Orientation::Top:
      return 3;
  }
  exit(1);
}

using Move = std::variant<int, Rotation>;

struct State {
  Pos3 position;
  Orientation orientation;
};

std::ostream& operator<<(std::ostream& out, const State& state) {
  out << "Pos{" << state.position.first << ',' << state.position.second << ',';
  switch (state.orientation) {
    case Orientation::Right: {
      out << '>';
      break;
    }
    case Orientation::Bottom: {
      out << 'v';
      break;
    }
    case Orientation::Left: {
      out << '<';
      break;
    }
    case Orientation::Top: {
      out << '^';
      break;
    }
  }
  out << '}';
  return out;
}

std::pair<Pos3, Pos3> GetBounds(const State& state, const Map& map) {
  switch (state.orientation) {
    case Orientation::Right: {
      const int row = state.position.first;
      return {{row, map.right_bounds.at(row)}, {row, map.left_bounds.at(row)}};
    }
    case Orientation::Bottom: {
      const int col = state.position.second;
      return {{map.bottom_bounds.at(col), col}, {map.top_bounds.at(col), col}};
    }
    case Orientation::Left: {
      const int row = state.position.first;
      return {{row, map.left_bounds.at(row)}, {row, map.right_bounds.at(row)}};
    }
    case Orientation::Top: {
      const int col = state.position.second;
      return {{map.top_bounds.at(col), col}, {map.bottom_bounds.at(col), col}};
    }
  }
  exit(1);
}

void Rotate(State& state, const Rotation& rotation) {
  switch (rotation) {
    case Rotation::Left: {
      state.orientation = PreviousOrientation(state.orientation);
      break;
    }
    case Rotation::Right: {
      state.orientation = NextOrientation(state.orientation);
      break;
    }
  }
}

void MoveForward(State& state, int steps, const Map& map) {
  for (int step = 0; step < steps; ++step) {
    const auto& [from, to] = GetBounds(state, map);
    Pos3 next_pos;
    if (state.position == from) {
      next_pos = to;
    } else {
      Pos3 delta = ToDelta(state.orientation);
      next_pos = {state.position.first + delta.first,
                  state.position.second + delta.second};
    }
    if (map.walls.contains(next_pos)) {
      break;
    }
    state.position = next_pos;
  }
}

State ProcessInstructions(const std::string& line, const Map& map) {
  State state = {{0, map.left_bounds.at(0)}, Orientation::Right};
  int left_cursor = 0;
  for (int right_cursor = 0; right_cursor < line.size(); ++right_cursor) {
    if (line.at(right_cursor) != 'R' && line.at(right_cursor) != 'L') {
      continue;
    }
    if (left_cursor != right_cursor) {
      MoveForward(
          state,
          std::stoi(line.substr(left_cursor, right_cursor - left_cursor)), map);
    }
    if (line.at(right_cursor) == 'R') {
      Rotate(state, Rotation::Right);
    } else {
      Rotate(state, Rotation::Left);
    }
    left_cursor = right_cursor + 1;
  }
  if (left_cursor < line.size()) {
    MoveForward(state, std::stoi(line.substr(left_cursor)), map);
  }
  return state;
}

std::ostream& operator<<(std::ostream& out, const Map& map) {
  int max_row = std::max_element(
                    map.bottom_bounds.begin(), map.bottom_bounds.end(),
                    [](auto lhs, auto rhs) { return lhs.second < rhs.second; })
                    ->second;
  for (int row = 0; row <= max_row; ++row) {
    for (int offset = 0;
         offset < std::to_string(max_row).size() - std::to_string(row).size();
         ++offset) {
      out << ' ';
    }
    out << row << ' ';
    for (int col = 0; col < map.left_bounds.at(row); ++col) {
      out << ' ';
    }
    for (int col = map.left_bounds.at(row); col <= map.right_bounds.at(row);
         ++col) {
      out << (map.walls.contains({row, col}) ? '#' : '.');
    }
    out << '\n';
  }
  out << "Row bounds: { ";
  for (const auto& [row, left_bound] : map.left_bounds) {
    out << row << ": " << left_bound << '>' << map.right_bounds.at(row) << ' ';
  }
  out << "}\nCol bounds: { ";
  for (const auto& [col, top_bound] : map.top_bounds) {
    out << col << ": " << top_bound << '>' << map.bottom_bounds.at(col) << ' ';
  }
  out << "}";
  return out;
}

Map Parse(std::istringstream& iss) {
  std::map<int, int> left_bounds;
  std::map<int, int> right_bounds;
  std::map<int, int> top_bounds;
  std::map<int, int> bottom_bounds;
  std::set<Pos3> walls;
  int row = 0;
  for (std::string line; std::getline(iss, line);) {
    if (line.empty()) {
      for (int col = left_bounds.at(row - 1); col <= right_bounds.at(row - 1);
           ++col) {
        bottom_bounds.insert({col, row - 1});
      }
      break;
    }
    int col = -1;
    for (char c : line) {
      ++col;
      if (c == ' ') {
        continue;
      }
      if (row == 0 || col < left_bounds.at(row - 1) ||
          right_bounds.at(row - 1) < col) {
        top_bounds.insert({col, row});
      }
      if (!left_bounds.contains(row)) {
        left_bounds.insert({row, col});
      }
      if (c == '#') {
        walls.insert({row, col});
      }
    }
    right_bounds.insert({row, col});
    if (row != 0 && left_bounds.at(row - 1) < left_bounds.at(row)) {
      for (int col = left_bounds.at(row - 1); col < left_bounds.at(row);
           ++col) {
        bottom_bounds.insert({col, row - 1});
      }
    }
    if (row != 0 && right_bounds.at(row) < right_bounds.at(row - 1)) {
      for (int col = right_bounds.at(row) + 1; col <= right_bounds.at(row - 1);
           ++col) {
        bottom_bounds.insert({col, row - 1});
      }
    }
    ++row;
  }
  return {left_bounds, right_bounds, top_bounds, bottom_bounds, walls};
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Map map = Parse(iss);
  // std::cerr << map << '\n';
  std::string line;
  std::getline(iss, line);
  State final_state = ProcessInstructions(line, map);
  // std::cerr << final_state << '\n';
  return std::to_string(kRowFactor * (final_state.position.first + 1) +
                        kColFactor * (final_state.position.second + 1) +
                        ToValue(final_state.orientation));
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
