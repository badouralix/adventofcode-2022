#include <bitset>
#include <cassert>
#include <chrono>
#include <functional>
#include <iostream>
#include <ostream>
#include <span>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

static constexpr int kWidth = 7;
static constexpr int kCountRocks = 2022;

using Row = std::bitset<kWidth>;
using Shape = std::vector<Row>;
using Tower = std::vector<Row>;
using Move = char;

static constexpr Row kLeftWall(64);
static constexpr Row kRightWall(1);

struct Rock {
  Shape shape;
  int height;
};

std::ostream& operator<<(std::ostream& out, const Rock& rock) {
  for (int index = rock.shape.size() - 1; index >= 0; --index) {
    out << rock.shape.at(index) << ' ' << rock.height + index << '\n';
  }
  return out;
}

std::ostream& operator<<(std::ostream& out, const Tower& tower) {
  for (int index = tower.size() - 1; index >= 0; --index) {
    out << tower.at(index) << ' ' << index << '\n';
  }
  return out;
}

class JetStreams {
 public:
  JetStreams(const std::string& jet_streams)
      : jet_streams_{jet_streams}, index_{0} {}

  char Next() {
    char result = jet_streams_.at(index_);
    index_ = (index_ + 1) % jet_streams_.size();
    return result;
  }

 private:
  std::string jet_streams_;
  int index_;
};

static const std::array<Shape, 5> kPrototypes = {
    Shape{16 + 8 + 4 + 2}, Shape{8, 16 + 4, 8}, Shape{16 + 8 + 4, 4, 4},
    Shape{16, 16, 16, 16}, Shape{16 + 8, 16 + 8}};

class Prototypes {
 public:
  Prototypes() : index_{0} {}

  Shape Next() {
    Shape result = kPrototypes.at(index_);
    index_ = (index_ + 1) % kPrototypes.size();
    return result;
  }

 private:
  int index_;
};

Row MoveLeft(Row row) { return row << 1; }
Row MoveRight(Row row) { return row >> 1; }

static const std::unordered_map<char, std::function<Row(Row)>> kMoveMap = {
    {'<', MoveLeft}, {'>', MoveRight}};

void LateralMove(const Tower& tower, Rock& rock, const Move& move) {
  Shape moved_shape;
  for (int index = 0; index < rock.shape.size(); ++index) {
    Row& row = rock.shape.at(index);
    if (move == '<' && (row & kLeftWall) != 0) {
      return;
    }
    if (move == '>' && (row & kRightWall) != 0) {
      return;
    }
    Row moved_row = kMoveMap.at(move)(row);
    moved_shape.emplace_back(moved_row);
    if (rock.height + index >= tower.size()) {
      continue;
    }
    if ((tower.at(rock.height + index) & moved_row) != 0) {
      return;
    }
  }
  rock.shape = std::move(moved_shape);
}

bool Fall(const Tower& tower, Rock& rock) {
  --rock.height;
  for (int index = 0; index < rock.shape.size(); ++index) {
    if (rock.height + index >= tower.size()) {
      break;
    }
    if ((tower.at(rock.height + index) & rock.shape.at(index)) != 0) {
      ++rock.height;
      return false;
    }
  }
  return true;
}

void Land(Tower& tower, const Rock& rock) {
  for (int index = 0; index < rock.shape.size(); ++index) {
    if (rock.height + index < tower.size()) {
      assert((rock.shape.at(index) & tower.at(rock.height + index)) == 0);
      tower.at(rock.height + index) |= rock.shape.at(index);
    } else {
      assert(rock.height + index == tower.size());
      tower.emplace_back(rock.shape.at(index));
    }
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  Prototypes prototypes;
  JetStreams jet_streams(input);
  Tower tower = {1 + 2 + 4 + 8 + 16 + 32 + 64};
  for (int rock_index = 0; rock_index < kCountRocks; ++rock_index) {
    Rock rock = {prototypes.Next(), static_cast<int>(tower.size()) + 3};
    // std::cerr << rock << '\n';
    LateralMove(tower, rock, jet_streams.Next());
    while (Fall(tower, rock)) {
      LateralMove(tower, rock, jet_streams.Next());
    }
    Land(tower, rock);
    // std::cerr << tower << '\n';
  }
  return std::to_string(tower.size() - 1);
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
