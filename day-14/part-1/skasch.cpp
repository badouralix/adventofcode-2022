#include <cassert>
#include <chrono>
#include <iostream>
#include <limits>
#include <map>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>

int kSandPosition = 500;

using Pos = std::pair<int, int>;

Pos operator+(const Pos& lhs, const Pos& rhs) {
  return {lhs.first + rhs.first, lhs.second + rhs.second};
}

std::ostream& operator<<(std::ostream& out, const Pos& pos) {
  out << "Pos{" << pos.first << ", " << pos.second << '}';
  return out;
}

class State {
 public:
  State(std::istringstream& iss)
      : blocked_{},
        last_open_{},
        min_x_{std::numeric_limits<int>::max()},
        max_x_{std::numeric_limits<int>::min()},
        max_y_{std::numeric_limits<int>::min()} {
    for (std::string line; std::getline(iss, line);) {
      ParseLine(line);
    }
  }

  bool DropSand(int x) {
    Pos pos = {x, last_open_.at(x)};
    while (true) {
      if (pos.second >= max_y_) {
        return true;
      }
      if (!blocked_.contains(pos + Pos{0, 1})) {
        pos = pos + Pos{0, 1};
        continue;
      }
      if (!blocked_.contains(pos + Pos{-1, 1})) {
        pos = pos + Pos{-1, 1};
        continue;
      }
      if (!blocked_.contains(pos + Pos{1, 1})) {
        pos = pos + Pos{1, 1};
        continue;
      }
      break;
    }
    Insert(pos);
    return false;
  }

  void Display() {
    for (int y = 0; y <= max_y_; ++y) {
      for (int x = min_x_; x <= max_x_; ++x) {
        if (blocked_.contains(Pos{x, y})) {
          std::cerr << '#';
        } else {
          std::cerr << '.';
        }
      }
      std::cerr << '\n';
    }
  }

 private:
  std::set<Pos> blocked_;
  std::map<int, int> last_open_;
  int min_x_;
  int max_x_;
  int max_y_;

  void UpdateBounds(const Pos& pos) {
    if (pos.first < min_x_) {
      min_x_ = pos.first;
    }
    if (pos.first > max_x_) {
      max_x_ = pos.first;
    }
    if (pos.second > max_y_) {
      max_y_ = pos.second;
    }
  }

  void Insert(const Pos& pos) {
    blocked_.insert(pos);
    if (!last_open_.contains(pos.first) ||
        pos.second <= last_open_.at(pos.first)) {
      last_open_.insert_or_assign(pos.first, pos.second - 1);
    }
  }

  void FillSection(const Pos& from, const Pos& to) {
    Pos delta;
    if (from.first != to.first) {
      assert(from.second == to.second);
      delta = from.first < to.first ? Pos{1, 0} : Pos{-1, 0};
    } else {
      delta = from.second < to.second ? Pos{0, 1} : Pos{0, -1};
    }
    for (Pos pos = from; pos != to + delta; pos = pos + delta) {
      Insert(pos);
    }
  }

  void ParseLine(const std::string& line) {
    int mid = line.find(',');
    int right = line.find(' ');
    Pos from = {std::stoi(line.substr(0, mid)),
                std::stoi(line.substr(mid + 1, right - mid - 1))};
    FillSection(from, from);
    UpdateBounds(from);
    int left = right + 4;
    while (left < line.size()) {
      mid = line.find(',', left);
      right = line.find(' ', left);
      if (right == -1) {
        right = line.size();
      }
      Pos to = {std::stoi(line.substr(left, mid - left)),
                std::stoi(line.substr(mid + 1, right - mid - 1))};
      FillSection(from, to);
      UpdateBounds(to);
      from = to;
      left = right + 4;
    }
  }
};

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  State state(iss);
  // state.Display();
  int sand_dropped = 0;
  while (true) {
    if (state.DropSand(kSandPosition)) {
      break;
    }
    ++sand_dropped;
    // state.Display();
  }
  return std::to_string(sand_dropped);
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
