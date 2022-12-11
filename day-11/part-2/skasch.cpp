#include <chrono>
#include <cstdint>
#include <deque>
#include <iostream>
#include <queue>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kRounds = 10000;

class Monkey {
 public:
  Monkey(std::istringstream& iss) : items_{}, inspections_{0} {
    for (std::string line; std::getline(iss, line);) {
      if (line.empty()) {
        break;
      }
      if (line.at(0) == 'M') {
        continue;
      }
      if (line.at(2) == 'S') {
        int left = 18;
        while (line.find(',', left) != std::string::npos) {
          int right = line.find(',', left);
          items_.push_back(std::stoi(line.substr(left, right - left)));
          left = right + 2;
        }
        items_.push_back(std::stoi(line.substr(left)));
        continue;
      }
      if (line.at(2) == 'O') {
        operation_ = line.substr(19);
        continue;
      }
      if (line.at(2) == 'T') {
        divisor_ = std::stoi(line.substr(21));
        continue;
      }
      if (line.at(7) == 't') {
        true_target_ = std::stoi(line.substr(29));
        continue;
      }
      if (line.at(7) == 'f') {
        false_target_ = std::stoi(line.substr(30));
        continue;
      }
      std::cerr << "Unexpected line when parsing a monkey: " << line << '\n';
      exit(1);
    }
  }

  void Display(int index) const {
    std::cerr << "Monkey " << index << ":\n";
    std::cerr << "  Items: ";
    for (int index = 0; index < items_.size(); ++index) {
      std::cerr << items_.at(index);
      if (index < items_.size() - 1) {
        std::cerr << ", ";
      }
    }
    std::cerr << '\n';
    std::cerr << "  Operation: " << operation_ << '\n';
    std::cerr << "  Test: divisible by " << divisor_ << '\n';
    std::cerr << "    If true: throw to monkey " << true_target_ << '\n';
    std::cerr << "    If false: throw to monkey " << false_target_ << '\n';
  }

  void DisplayItems(int index) const {
    std::cerr << "Monkey " << index << ": ";
    for (int index = 0; index < items_.size(); ++index) {
      std::cerr << items_.at(index);
      if (index < items_.size() - 1) {
        std::cerr << ", ";
      }
    }
    std::cerr << '\n';
  }

  void Turn(std::vector<Monkey>& monkeys, std::int64_t base) {
    while (!items_.empty()) {
      InspectItem(monkeys, base);
    }
  }

  std::int64_t GetDivisor() const { return divisor_; }
  int GetInspections() const { return inspections_; }

 private:
  std::deque<std::int64_t> items_;
  std::string operation_;
  std::int64_t divisor_;
  int true_target_;
  int false_target_;
  int inspections_;

  int UpdateWorryLevel(std::int64_t item, std::int64_t base) {
    std::int64_t lhs = item;
    char op = operation_.at(4);
    std::int64_t rhs;
    if (operation_.at(6) == 'o') {
      rhs = item;
    } else {
      rhs = std::stoi(operation_.substr(6));
    }
    switch (op) {
      case '+':
        return (lhs + rhs) % base;
      case '*':
        return (lhs * rhs) % base;
      default:
        std::cerr << "Invalid operator " << op << '\n';
        exit(1);
    }
  }

  void AddItem(std::int64_t item) { items_.push_back(item); }

  void InspectItem(std::vector<Monkey>& monkeys, std::int64_t base) {
    std::int64_t item = UpdateWorryLevel(items_.front(), base);
    items_.pop_front();
    if (item % divisor_ == 0) {
      monkeys.at(true_target_).AddItem(item);
    } else {
      monkeys.at(false_target_).AddItem(item);
    }
    ++inspections_;
  }
};

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::vector<Monkey> monkeys;
  while (!iss.eof()) {
    monkeys.emplace_back(iss);
  }
  std::int64_t base = 1;
  for (const auto& monkey : monkeys) {
    base *= monkey.GetDivisor();
  }
  for (int round = 0; round < kRounds; ++round) {
    for (int index = 0; index < monkeys.size(); ++index) {
      monkeys.at(index).Turn(monkeys, base);
    }
  }
  std::priority_queue<std::int64_t, std::vector<std::int64_t>,
                      std::greater<std::int64_t>>
      max2;
  for (int index = 0; index < monkeys.size(); ++index) {
    max2.push(monkeys.at(index).GetInspections());
    if (max2.size() > 2) {
      max2.pop();
    }
  }
  std::int64_t score = max2.top();
  max2.pop();
  score *= max2.top();
  return std::to_string(score);
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
