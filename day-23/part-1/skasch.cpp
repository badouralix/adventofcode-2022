#include <algorithm>
#include <chrono>
#include <deque>
#include <iostream>
#include <map>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kRounds = 10;

enum class Direction {
  North,
  South,
  West,
  East,
};

static const std::vector<Direction> kDirections = {
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
};

std::ostream& operator<<(std::ostream& out, const Direction& direction) {
  switch (direction) {
    case Direction::North: {
      out << 'N';
      break;
    }
    case Direction::South: {
      out << 'S';
      break;
    }
    case Direction::West: {
      out << 'W';
      break;
    }
    case Direction::East: {
      out << 'E';
      break;
    }
  }
  return out;
}

class DirectionsManager {
 public:
  DirectionsManager() : directions_{kDirections.begin(), kDirections.end()} {}

  void Rotate() {
    std::rotate(directions_.begin(), directions_.begin() + 1,
                directions_.end());
  }

  std::deque<Direction>::const_iterator begin() const {
    return directions_.begin();
  }
  std::deque<Direction>::const_iterator end() const {
    return directions_.end();
  }

 private:
  std::deque<Direction> directions_;
};

using Pos = std::pair<int, int>;

int X(const Pos& pos) { return pos.first; }
int Y(const Pos& pos) { return pos.second; }

std::ostream& operator<<(std::ostream& out, const Pos& pos) {
  out << '(' << X(pos) << ',' << Y(pos) << ')';
  return out;
}

Pos operator+(const Pos& lhs, const Pos& rhs) {
  return {X(lhs) + X(rhs), Y(lhs) + Y(rhs)};
}

static const std::vector<Pos> kAroundDeltas = {
    {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1},
};
static const std::map<Direction, std::vector<Pos>> kDirectionDeltas = {
    {Direction::North, {{-1, -1}, {-1, 0}, {-1, 1}}},
    {Direction::South, {{1, -1}, {1, 0}, {1, 1}}},
    {Direction::West, {{-1, -1}, {0, -1}, {1, -1}}},
    {Direction::East, {{-1, 1}, {0, 1}, {1, 1}}},
};
static const std::map<Direction, Pos> kDirectionDelta = {
    {Direction::North, {-1, 0}},
    {Direction::South, {1, 0}},
    {Direction::West, {0, -1}},
    {Direction::East, {0, 1}},
};

using Map = std::set<Pos>;
using Movements = std::map<Pos, Pos>;
using Counts = std::map<Pos, int>;

std::tuple<int, int, int, int> GetBounds(const Map& map) {
  int min_row = std::numeric_limits<int>::max();
  int max_row = std::numeric_limits<int>::min();
  int min_col = std::numeric_limits<int>::max();
  int max_col = std::numeric_limits<int>::min();
  for (const auto& [row, col] : map) {
    min_row = std::min(min_row, row);
    max_row = std::max(max_row, row);
    min_col = std::min(min_col, col);
    max_col = std::max(max_col, col);
  }
  return {min_row, max_row, min_col, max_col};
}

std::ostream& operator<<(std::ostream& out, const Map& map) {
  auto [min_row, max_row, min_col, max_col] = GetBounds(map);
  for (int row = min_row; row <= max_row; ++row) {
    for (int col = min_col; col <= max_col; ++col) {
      out << (map.contains({row, col}) ? '#' : '.');
    }
    out << '\n';
  }
  return out;
}

int CountAround(const Pos& pos, const Map& map) {
  int count = 0;
  for (const Pos& delta : kAroundDeltas) {
    if (map.contains(pos + delta)) {
      ++count;
    }
  }
  return count;
}

int CountDirection(const Pos& pos, const Direction& direction, const Map& map) {
  int count = 0;
  for (const Pos& delta : kDirectionDeltas.at(direction)) {
    if (map.contains(pos + delta)) {
      ++count;
    }
  }
  return count;
}

void PrepareMove(const Pos& pos, const Map& map,
                 const DirectionsManager& directions_manager,
                 Movements& movements, Counts& counts) {
  if (CountAround(pos, map) == 0) {
    return;
  }
  for (const Direction& direction : directions_manager) {
    if (CountDirection(pos, direction, map) != 0) {
      continue;
    }
    Pos next_pos = pos + kDirectionDelta.at(direction);
    movements.insert({pos, next_pos});
    if (counts.contains(next_pos)) {
      ++counts.at(next_pos);
    } else {
      counts.insert({next_pos, 1});
    }
    break;
  }
}

void ApplyMoves(Map& map, const Movements& movements, const Counts& counts) {
  for (const auto& [from, to] : movements) {
    if (counts.at(to) > 1) {
      continue;
    }
    map.erase(from);
    map.insert(to);
  }
}

void Round(Map& map, DirectionsManager& directions_manager) {
  Movements movements;
  Counts counts;
  for (const Pos& pos : map) {
    PrepareMove(pos, map, directions_manager, movements, counts);
  }
  ApplyMoves(map, movements, counts);
  directions_manager.Rotate();
}

Map Parse(const std::string& input) {
  Map map;
  std::istringstream iss(input);
  std::string line;
  for (int row = 0; std::getline(iss, line); ++row) {
    for (int col = 0; const auto& c : line) {
      if (c == '#') {
        map.insert({row, col});
      }
      ++col;
    }
  }
  return map;
}

int GetScore(const Map& map) {
  auto [min_row, max_row, min_col, max_col] = GetBounds(map);
  return (max_row - min_row + 1) * (max_col - min_col + 1) - map.size();
}

std::string Run(const std::string& input) {
  // Your code goes here
  Map map = Parse(input);
  DirectionsManager directions_manager;
  for (int round = 0; round < kRounds; ++round) {
    Round(map, directions_manager);
  }
  return std::to_string(GetScore(map));
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
