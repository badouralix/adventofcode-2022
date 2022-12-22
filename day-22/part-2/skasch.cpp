#include <cassert>
#include <chrono>
#include <iostream>
#include <map>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

inline constexpr int kSize = 50;
inline constexpr int kHalfSize = kSize / 2;
inline constexpr int kRowFactor = 1000;
inline constexpr int kColFactor = 4;

using Pos3 = std::tuple<int, int, int>;
using Pos2 = std::pair<int, int>;

int X(const Pos3& pos) { return std::get<0>(pos); }
int X(const Pos2& pos) { return pos.first; }
int Y(const Pos3& pos) { return std::get<1>(pos); }
int Y(const Pos2& pos) { return pos.second; }
int Z(const Pos3& pos) { return std::get<2>(pos); }
int& X(Pos3& pos) { return std::get<0>(pos); }
int& X(Pos2& pos) { return pos.first; }
int& Y(Pos3& pos) { return std::get<1>(pos); }
int& Y(Pos2& pos) { return pos.second; }
int& Z(Pos3& pos) { return std::get<2>(pos); }

Pos3 operator+(const Pos3& lhs, const Pos3& rhs) {
  return {
      X(lhs) + X(rhs),
      Y(lhs) + Y(rhs),
      Z(lhs) + Z(rhs),
  };
}

void operator+=(Pos3& lhs, const Pos3& rhs) {
  X(lhs) += X(rhs);
  Y(lhs) += Y(rhs);
  Z(lhs) += Z(rhs);
}

Pos3 operator-(const Pos3& lhs, const Pos3& rhs) {
  return {
      X(lhs) - X(rhs),
      Y(lhs) - Y(rhs),
      Z(lhs) - Z(rhs),
  };
}

Pos2 operator-(const Pos2& lhs, const Pos2& rhs) {
  return {
      X(lhs) - X(rhs),
      Y(lhs) - Y(rhs),
  };
}

Pos3 operator-(const Pos3& pos) {
  return {
      -X(pos),
      -Y(pos),
      -Z(pos),
  };
}

Pos3 operator*(int factor, const Pos3& pos) {
  return {
      factor * X(pos),
      factor * Y(pos),
      factor * Z(pos),
  };
}

// vector product
Pos3 operator^(const Pos3& lhs, const Pos3& rhs) {
  return {
      Y(lhs) * Z(rhs) - Z(lhs) * Y(rhs),
      Z(lhs) * X(rhs) - X(lhs) * Z(rhs),
      X(lhs) * Y(rhs) - Y(lhs) * X(rhs),
  };
}

std::ostream& operator<<(std::ostream& out, const Pos2& pos) {
  out << '(' << X(pos) << ',' << Y(pos) << ')';
  return out;
}

std::ostream& operator<<(std::ostream& out, const Pos3& pos) {
  out << '(' << X(pos) << ',' << Y(pos) << ',' << Z(pos) << ')';
  return out;
}

struct Base {
  Pos3 reference;
  Pos3 x_vector;
  Pos3 y_vector;
  Pos3 z_vector;
};

std::ostream& operator<<(std::ostream& out, const Base& base) {
  out << "Base{" << base.reference << ", " << base.x_vector << ", "
      << base.y_vector << ", " << base.z_vector << '}';
  return out;
}

using Walls = std::set<Pos3>;

struct Map {
  Walls walls;
  std::map<Pos3, Pos2> _3d_to_2d;
  std::map<Pos2, Pos3> _2d_to_3d;
};

void ParseFace(const std::vector<std::string>& lines, int row, int col,
               const Base& base, Map& map) {
  for (int drow = 0; drow < kSize; ++drow) {
    for (int dcol = 0; dcol < kSize; ++dcol) {
      Pos3 pos = base.reference + dcol * base.x_vector + drow * base.y_vector;
      map._3d_to_2d.insert({pos, {row + drow, col + dcol}});
      map._2d_to_3d.insert({{row + drow, col + dcol}, pos});
      if (lines.at(row + drow).at(col + dcol) == '#') {
        map.walls.insert(pos);
      }
    }
  }
}

Base RotateRight(const Base& base) {
  return {
      base.reference + kSize * base.x_vector + base.z_vector,
      base.z_vector,
      base.y_vector,
      -base.x_vector,
  };
}

Base RotateDown(const Base& base) {
  return {
      base.reference + kSize * base.y_vector + base.z_vector,
      base.x_vector,
      base.z_vector,
      -base.y_vector,
  };
}

Base RotateLeft(const Base& base) {
  return {
      base.reference + kSize * base.z_vector - base.x_vector,
      -base.z_vector,
      base.y_vector,
      base.x_vector,
  };
}

static Base GetInitialBase() {
  return {
      {-kHalfSize, -kHalfSize, -kHalfSize - 1},
      {1, 0, 0},
      {0, 1, 0},
      {0, 0, 1},
  };
}

Map ParseLines(const std::vector<std::string>& lines) {
  Map map;
  Base base = GetInitialBase();
  int common_col = -1;
  Base common_base;
  for (int row = 0; row < lines.size(); row += kSize) {
    if (common_col == -1) {
      for (int col = 0; col < lines.at(row).size(); col += kSize) {
        if (lines.at(row).at(col) == ' ') {
          continue;
        }
        if (col < lines.at(row + kSize).size() &&
            lines.at(row + kSize).at(col) != ' ') {
          common_col = col;
          common_base = RotateDown(base);
        }
        ParseFace(lines, row, col, base, map);
        base = RotateRight(base);
      }
    } else {
      int col = common_col;
      base = common_base;
      common_col = -1;
      if (row + kSize < lines.size() && col < lines.at(row + kSize).size() &&
          lines.at(row + kSize).at(col) != ' ') {
        common_col = col;
        common_base = RotateDown(base);
      }
      ParseFace(lines, row, col, base, map);
      Base left_base = base;
      for (int left_col = col - kSize; left_col >= 0; left_col -= kSize) {
        if (lines.at(row).at(left_col) == ' ') {
          break;
        }
        left_base = RotateLeft(left_base);
        ParseFace(lines, row, left_col, left_base, map);
        if (common_col == -1 && row + kSize < lines.size() &&
            left_col < lines.at(row + kSize).size() &&
            lines.at(row + kSize).at(left_col) != ' ') {
          common_col = left_col;
          common_base = RotateDown(left_base);
        }
      }
      for (int right_col = col + kSize; right_col < lines.at(row).size();
           right_col += kSize) {
        base = RotateRight(base);
        ParseFace(lines, row, right_col, base, map);
        if (common_col == -1 && row + kSize < lines.size() &&
            right_col < lines.at(row + kSize).size() &&
            lines.at(row + kSize).at(right_col) != ' ') {
          common_col = right_col;
          common_base = RotateDown(base);
        }
      }
    }
  }
  return map;
}

enum class Rotation {
  Left,
  Right,
};

void Rotate(Base& base, const Rotation& rotation) {
  switch (rotation) {
    case Rotation::Left: {
      std::swap(base.x_vector, base.y_vector);
      base.x_vector = -base.x_vector;
      break;
    }
    case Rotation::Right: {
      std::swap(base.x_vector, base.y_vector);
      base.y_vector = -base.y_vector;
      break;
    }
    default:
      exit(1);
  }
}

void MoveForward(Base& base, int steps, const Map& map) {
  for (int step = 0; step < steps; ++step) {
    bool rotated = false;
    Pos3 next_pos = base.reference + base.x_vector;
    if (!map._3d_to_2d.contains(next_pos)) {
      next_pos += base.z_vector;
      rotated = true;
    }
    assert(map._3d_to_2d.contains(next_pos));
    if (map.walls.contains(next_pos)) {
      break;
    }
    if (rotated) {
      base = {
          next_pos,
          base.z_vector,
          base.y_vector,
          -base.x_vector,
      };
    } else {
      base.reference = next_pos;
    }
  }
}

Base ProcessInstructions(const std::string& line, const Map& map) {
  Base base = GetInitialBase();
  int left_cursor = 0;
  for (int right_cursor = 0; right_cursor < line.size(); ++right_cursor) {
    if (line.at(right_cursor) != 'R' && line.at(right_cursor) != 'L') {
      continue;
    }
    if (left_cursor != right_cursor) {
      MoveForward(
          base, std::stoi(line.substr(left_cursor, right_cursor - left_cursor)),
          map);
    }
    if (line.at(right_cursor) == 'R') {
      Rotate(base, Rotation::Right);
    } else {
      Rotate(base, Rotation::Left);
    }
    left_cursor = right_cursor + 1;
  }
  if (left_cursor < line.size()) {
    MoveForward(base, std::stoi(line.substr(left_cursor)), map);
  }
  return base;
}

Pos2 GetOrientation(const Base& base, const Map& map) {
  if (map._3d_to_2d.contains(base.reference + base.x_vector)) {
    return map._3d_to_2d.at(base.reference + base.x_vector) -
           map._3d_to_2d.at(base.reference);
  }
  return map._3d_to_2d.at(base.reference) -
         map._3d_to_2d.at(base.reference - base.x_vector);
}

int GetOrientationScore(const Pos2& orientation) {
  if (X(orientation) == 1 && Y(orientation) == 0) {
    return 1;
  } else if (X(orientation) == 0 && Y(orientation) == 1) {
    return 0;
  } else if (X(orientation) == -1 && Y(orientation) == 0) {
    return 3;
  } else if (X(orientation) == 0 && Y(orientation) == -1) {
    return 2;
  }
  exit(1);
}

void DisplayFaces(const Map& map) {
  int z = -kHalfSize - 1;
  std::cerr << "Face z = " << z << '\n';
  for (int x = -kHalfSize; x < kHalfSize; ++x) {
    for (int y = -kHalfSize; y < kHalfSize; ++y) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
  z = kHalfSize;
  std::cerr << "Face z = " << z << '\n';
  for (int x = -kHalfSize; x < kHalfSize; ++x) {
    for (int y = -kHalfSize; y < kHalfSize; ++y) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
  int x = -kHalfSize - 1;
  std::cerr << "Face x = " << x << '\n';
  for (int y = -kHalfSize; y < kHalfSize; ++y) {
    for (int z = -kHalfSize; z < kHalfSize; ++z) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
  x = kHalfSize;
  std::cerr << "Face x = " << x << '\n';
  for (int y = -kHalfSize; y < kHalfSize; ++y) {
    for (int z = -kHalfSize; z < kHalfSize; ++z) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
  int y = -kHalfSize - 1;
  std::cerr << "Face y = " << y << '\n';
  for (int z = -kHalfSize; z < kHalfSize; ++z) {
    for (int x = -kHalfSize; x < kHalfSize; ++x) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
  y = kHalfSize;
  std::cerr << "Face y = " << y << '\n';
  for (int z = -kHalfSize; z < kHalfSize; ++z) {
    for (int x = -kHalfSize; x < kHalfSize; ++x) {
      assert(map._3d_to_2d.contains({x, y, z}));
      std::cerr << (map.walls.contains({x, y, z}) ? '#' : '.');
    }
    std::cerr << '\n';
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::vector<std::string> lines;
  for (std::string line; std::getline(iss, line);) {
    if (line.empty()) {
      break;
    }
    lines.emplace_back(line);
  }
  Map map = ParseLines(lines);
  // DisplayFaces(map);
  std::string line;
  std::getline(iss, line);
  Base position = ProcessInstructions(line, map);
  Pos2 pos2d = map._3d_to_2d.at(position.reference);
  return std::to_string(kRowFactor * (X(pos2d) + 1) +
                        kColFactor * (Y(pos2d) + 1) +
                        GetOrientationScore(GetOrientation(position, map)));
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
