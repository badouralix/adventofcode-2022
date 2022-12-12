#include <chrono>
#include <cmath>
#include <deque>
#include <functional>
#include <iostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <vector>

using Pos = std::pair<int, int>;

int ToElevation(const char c) { return c - 'a'; }

class Map {
 public:
  Map(const std::string& input) : elevation_map_{} {
    std::istringstream iss(input);
    int row = 0;
    for (std::string line; std::getline(iss, line);) {
      std::vector<int> elevation_row;
      for (int col = 0; col < line.size(); ++col) {
        switch (line.at(col)) {
          case 'S':
            start_ = {row, col};
            [[fallthrough]];
          case 'a':
            elevation_row.push_back(0);
            break;
          case 'E':
            end_ = {row, col};
            [[fallthrough]];
          case 'z':
            elevation_row.push_back(25);
            break;
          default:
            elevation_row.push_back(ToElevation(line.at(col)));
            break;
        }
      }
      elevation_map_.emplace_back(std::move(elevation_row));
      ++row;
    }
  }

  int FindBestStartingPoint() {
    std::deque<BFSState> deque;
    deque.emplace_back(end_, 0);
    std::set<Pos> visited;
    BFSState state;
    while (!deque.empty()) {
      state = deque.front();
      deque.pop_front();
      if (visited.contains(state.pos)) {
        continue;
      }
      visited.insert(state.pos);
      if (ElevationAt(state.pos) == 0) {
        return state.steps;
      }
      for (const Pos& neighbor :
           std::vector<Pos>{{state.pos.first + 1, state.pos.second},
                            {state.pos.first - 1, state.pos.second},
                            {state.pos.first, state.pos.second + 1},
                            {state.pos.first, state.pos.second - 1}}) {
        if (visited.contains(neighbor) || !IsValid(neighbor) ||
            ElevationAt(neighbor) < ElevationAt(state.pos) - 1) {
          continue;
        }
        deque.emplace_back(neighbor, state.steps + 1);
      }
    }
    std::cerr << "No valid path found.\n";
    exit(1);
  }

 private:
  Pos start_;
  Pos end_;
  std::vector<std::vector<int>> elevation_map_;

  struct BFSState {
    Pos pos;
    int steps;
  };

  bool IsValid(const Pos& pos) const {
    return pos.first >= 0 && pos.first < elevation_map_.size() &&
           pos.second >= 0 && pos.second < elevation_map_.at(pos.first).size();
  }

  int ElevationAt(const Pos& pos) const {
    return elevation_map_.at(pos.first).at(pos.second);
  }
};

std::string Run(const std::string& input) {
  Map map(input);
  return std::to_string(map.FindBestStartingPoint());
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
