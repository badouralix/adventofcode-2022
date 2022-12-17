#include <chrono>
#include <iostream>
#include <limits>
#include <map>
#include <optional>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

int kMinRow = 0;
int kMaxRow = 4000000;
int kMinCol = 0;
int kMaxCol = 4000000;
std::int64_t kFrequencyFactor = 4000000;

using Pos = std::pair<int, int>;

std::ostream& operator<<(std::ostream& out, const Pos& pos) {
  out << "Pos{" << pos.first << ", " << pos.second << "}";
  return out;
}

class Range {
 public:
  Range(int from, int to) : from_{from}, to_{to} {
    if (from > to) {
      std::cerr << "Invalid range " << from << " -> " << to << ".\n";
      exit(1);
    }
  }

  int GetFrom() const { return from_; }
  int GetTo() const { return to_; }

 private:
  int from_;
  int to_;

  friend std::ostream& operator<<(std::ostream& out, const Range& range) {
    out << "Range{" << range.from_ << ", " << range.to_ << "}";
    return out;
  }
};

struct Sensor {
  Pos position;
  Pos beacon;
};

int Distance(const Pos& lhs, const Pos& rhs) {
  return std::abs(rhs.first - lhs.first) + std::abs(rhs.second - lhs.second);
}

std::optional<Range> Project(const Sensor& sensor, int row) {
  int distance = Distance(sensor.position, sensor.beacon);
  int row_diff = std::abs(sensor.position.second - row);
  if (row_diff > distance) {
    return std::nullopt;
  }
  int radius = distance - row_diff;
  return Range(sensor.position.first - radius, sensor.position.first + radius);
}

std::map<int, int> InitializeDeltaMap(const std::vector<Range>& ranges) {
  std::map<int, int> deltas{};
  for (const Range& range : ranges) {
    if (deltas.contains(range.GetFrom())) {
      ++deltas.at(range.GetFrom());
    } else {
      deltas.insert({range.GetFrom(), 1});
    }
    if (deltas.at(range.GetFrom()) == 0) {
      deltas.erase(range.GetFrom());
    }
    if (deltas.contains(range.GetTo() + 1)) {
      --deltas.at(range.GetTo() + 1);
    } else {
      deltas.insert({range.GetTo() + 1, -1});
    }
    if (deltas.at(range.GetTo() + 1) == 0) {
      deltas.erase(range.GetTo() + 1);
    }
  }
  return deltas;
}

std::optional<int> FindEmpty(const std::vector<Range>& ranges) {
  std::map<int, int> deltas = InitializeDeltaMap(ranges);
  int block_depth = 0;
  for (const auto& [right, delta] : deltas) {
    if (right > kMaxCol) {
      return std::nullopt;
    }
    block_depth += delta;
    if (block_depth < 0) {
      std::cerr << "Invalid depth: " << block_depth;
      exit(1);
    }
    if (right < kMinCol) {
      continue;
    }
    if (block_depth == 0) {
      return right;
    }
  }
  return std::nullopt;
}

Sensor ParseLine(const std::string& line) {
  int left = 12;
  int right = line.find(',', left);
  int pos_x = std::stoi(line.substr(left, right - left));
  left = right + 4;
  right = line.find(':', left);
  int pos_y = std::stoi(line.substr(left, right - left));
  left = right + 25;
  right = line.find(',', left);
  int beacon_x = std::stoi(line.substr(left, right - left));
  left = right + 4;
  int beacon_y = std::stoi(line.substr(left));
  return Sensor{Pos{pos_x, pos_y}, Pos{beacon_x, beacon_y}};
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::vector<Sensor> sensors;
  std::set<int> diag1s;
  std::set<int> diag1s_twice;
  std::set<int> diag2s;
  std::set<int> diag2s_twice;
  for (std::string line; std::getline(iss, line);) {
    Sensor sensor = ParseLine(line);
    sensors.emplace_back(sensor);
    int border = Distance(sensor.position, sensor.beacon) + 1;
    int diag1_1 = sensor.position.second - border - sensor.position.first;
    int diag1_2 = sensor.position.second + border - sensor.position.first;
    int diag2_1 = sensor.position.second - border + sensor.position.first;
    int diag2_2 = sensor.position.second + border + sensor.position.first;
    if (diag1s.contains(diag1_1)) {
      diag1s_twice.insert(diag1_1);
    } else {
      diag1s.insert(diag1_1);
    }
    if (diag1s.contains(diag1_2)) {
      diag1s_twice.insert(diag1_2);
    } else {
      diag1s.insert(diag1_2);
    }
    if (diag2s.contains(diag2_1)) {
      diag2s_twice.insert(diag2_1);
    } else {
      diag2s.insert(diag2_1);
    }
    if (diag2s.contains(diag2_2)) {
      diag2s_twice.insert(diag2_2);
    } else {
      diag2s.insert(diag2_2);
    }
  }
  for (int diag1 : diag1s_twice) {
    for (int diag2 : diag2s_twice) {
      if ((diag1 + diag2) % 2 != 0) {
        continue;
      }
      int x = (diag2 - diag1) / 2;
      int y = (diag1 + diag2) / 2;
      Pos beacon = {x, y};
      if (diag1s_twice.size() == 1 && diag2s_twice.size() == 1) {
        return std::to_string(static_cast<std::int64_t>(x) * kFrequencyFactor +
                              static_cast<std::int64_t>(y));
      }
      if (x < kMinCol || kMaxCol < x || y < kMinRow || kMaxRow < y) {
        continue;
      }
      bool is_valid = true;
      for (const auto& sensor : sensors) {
        if (Distance(sensor.position, beacon) <=
            Distance(sensor.position, sensor.beacon)) {
          is_valid = false;
          break;
        }
      }
      if (is_valid) {
        return std::to_string(static_cast<std::int64_t>(x) * kFrequencyFactor +
                              static_cast<std::int64_t>(y));
      }
    }
  }
  exit(1);
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
