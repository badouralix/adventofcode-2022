#include <bitset>
#include <cassert>
#include <chrono>
#include <cstdint>
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
static constexpr std::int64_t kCountRocks = 1000000000000;

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

  int GetIndex() const { return index_; }

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

  int GetIndex() const { return index_; }

 private:
  int index_;
};

Row MoveLeft(Row row) { return row << 1; }
Row MoveRight(Row row) { return row >> 1; }

static const std::unordered_map<char, std::function<Row(Row)>> kMoveMap = {
    {'<', MoveLeft}, {'>', MoveRight}};

struct GlobalState {
  Prototypes prototypes;
  JetStreams jet_streams;
  Tower tower;
  std::int64_t rock_index;
  std::int64_t previous_rock_index;
  std::int64_t additional_height;
  int previous_tower_height;
  int previous_prototype_index;
  int previous_rock_drop;
};

std::ostream& operator<<(std::ostream& out, const GlobalState& state) {
  out << "Prototype index: " << state.prototypes.GetIndex()
      << " | Jet streams index: " << state.jet_streams.GetIndex()
      << " | Tower height: " << state.tower.size()
      << " | Rock index: " << state.rock_index
      << " | Previous rock index: " << state.previous_rock_index
      << " | Additional height: " << state.additional_height
      << " | Previous tower height: " << state.previous_tower_height
      << " | Previous prototype height: " << state.previous_prototype_index
      << " | Previous rock drop: " << state.previous_rock_drop;
  return out;
}

void LateralMove(GlobalState& state, Rock& rock, const Move& move,
                 int base_height) {
  Shape moved_shape;
  bool set_shape = true;
  for (int index = 0; index < rock.shape.size(); ++index) {
    Row& row = rock.shape.at(index);
    if (move == '<' && (row & kLeftWall) != 0) {
      set_shape = false;
      break;
    }
    if (move == '>' && (row & kRightWall) != 0) {
      set_shape = false;
      break;
    }
    Row moved_row = kMoveMap.at(move)(row);
    moved_shape.emplace_back(moved_row);
    if (rock.height + index >= state.tower.size()) {
      continue;
    }
    if ((state.tower.at(rock.height + index) & moved_row) != 0) {
      set_shape = false;
      break;
    }
  }
  if (set_shape) {
    rock.shape = std::move(moved_shape);
  }
  if (state.jet_streams.GetIndex() == 0) {
    if (state.prototypes.GetIndex() == state.previous_prototype_index &&
        base_height - rock.height == state.previous_rock_drop) {
      std::int64_t rock_index_delta =
          state.rock_index - state.previous_rock_index;
      std::int64_t tower_height_delta =
          state.tower.size() - state.previous_tower_height;
      std::int64_t repetitions =
          (kCountRocks - state.rock_index) / rock_index_delta;
      state.additional_height = repetitions * tower_height_delta;
      state.rock_index += repetitions * rock_index_delta;
    } else {
      state.previous_prototype_index = state.prototypes.GetIndex();
      state.previous_rock_drop = base_height - rock.height;
      state.previous_rock_index = state.rock_index;
      state.previous_tower_height = state.tower.size();
    }
  }
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
  GlobalState state = {Prototypes(),
                       JetStreams(input),
                       {1 + 2 + 4 + 8 + 16 + 32 + 64},
                       0,
                       0,
                       0,
                       1,
                       -1,
                       0};
  for (; state.rock_index < kCountRocks; ++state.rock_index) {
    Rock rock = {state.prototypes.Next(),
                 static_cast<int>(state.tower.size()) + 3};
    int base_height = rock.height;
    LateralMove(state, rock, state.jet_streams.Next(), base_height);
    while (Fall(state.tower, rock)) {
      LateralMove(state, rock, state.jet_streams.Next(), base_height);
    }
    Land(state.tower, rock);
  }
  return std::to_string(static_cast<std::int64_t>(state.tower.size()) - 1 +
                        state.additional_height);
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
