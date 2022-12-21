#include <chrono>
#include <cstdint>
#include <iostream>
#include <map>
#include <ostream>
#include <span>
#include <sstream>
#include <string>
#include <variant>

inline constexpr int kMathThreshold = 14;

using Op = char;
using Val = std::int64_t;

struct ValMonkey {
  std::string name;
  Val value;
};

std::ostream& operator<<(std::ostream& out, const ValMonkey& monkey) {
  out << "ValMonkey{" << monkey.name << ": " << monkey.value << "}";
  return out;
}

struct MathMonkey {
  std::string name;
  std::string lhs;
  Op op;
  std::string rhs;
};

struct RootMonkey {
  std::string lhs;
  std::string rhs;
};

std::ostream& operator<<(std::ostream& out, const MathMonkey& monkey) {
  out << "MathMonkey{" << monkey.name << ": " << monkey.lhs << " " << monkey.op
      << " " << monkey.rhs << "}";
  return out;
}

using Monkey = std::variant<ValMonkey, MathMonkey>;
using Memory = std::map<std::string, Val>;
using Triggers = std::multimap<std::string, MathMonkey>;

Val Compute(const Memory& memory, const MathMonkey& monkey) {
  switch (monkey.op) {
    case '+': {
      return memory.at(monkey.lhs) + memory.at(monkey.rhs);
    }
    case '-': {
      return memory.at(monkey.lhs) - memory.at(monkey.rhs);
    }
    case '*': {
      return memory.at(monkey.lhs) * memory.at(monkey.rhs);
    }
    case '/': {
      return memory.at(monkey.lhs) / memory.at(monkey.rhs);
    }
    default:
      std::cerr << "Invalid computation for monkey " << monkey << '\n';
      exit(1);
  }
}

Val ComputeLhs(const Memory& memory, const MathMonkey& monkey) {
  switch (monkey.op) {
    case '+': {
      return memory.at(monkey.name) - memory.at(monkey.rhs);
    }
    case '-': {
      return memory.at(monkey.name) + memory.at(monkey.rhs);
    }
    case '*': {
      return memory.at(monkey.name) / memory.at(monkey.rhs);
    }
    case '/': {
      return memory.at(monkey.name) * memory.at(monkey.rhs);
    }
    default:
      std::cerr << "Invalid LHS computation for monkey " << monkey << '\n';
      exit(1);
  }
}

Val ComputeRhs(const Memory& memory, const MathMonkey& monkey) {
  switch (monkey.op) {
    case '+': {
      return memory.at(monkey.name) - memory.at(monkey.lhs);
    }
    case '-': {
      return memory.at(monkey.lhs) - memory.at(monkey.name);
    }
    case '*': {
      return memory.at(monkey.name) / memory.at(monkey.lhs);
    }
    case '/': {
      return memory.at(monkey.lhs) / memory.at(monkey.name);
    }
    default:
      std::cerr << "Invalid RHS computation for monkey " << monkey << '\n';
      exit(1);
  }
}

Monkey Parse(const std::string& line) {
  if (line.size() > kMathThreshold) {
    std::string name = line.substr(0, 4);
    return MathMonkey{
        name,
        line.substr(6, 4),
        name == "root" ? '=' : line.at(11),
        line.substr(13),
    };
  }
  return ValMonkey{line.substr(0, 4), std::stoi(line.substr(6))};
}

void UpdateState(const Monkey& monkey, Triggers& triggers, Memory& memory);

void Propagate(const MathMonkey& monkey, Triggers& triggers, Memory& memory,
               bool add_triggers) {
  if (monkey.op == '=') {
    if (memory.contains(monkey.lhs) && !memory.contains(monkey.rhs)) {
      UpdateState(ValMonkey{monkey.rhs, memory.at(monkey.lhs)}, triggers,
                  memory);
    } else if (!memory.contains(monkey.lhs) && memory.contains(monkey.rhs)) {
      UpdateState(ValMonkey{monkey.lhs, memory.at(monkey.rhs)}, triggers,
                  memory);
    } else if (add_triggers) {
      triggers.insert({monkey.lhs, monkey});
      triggers.insert({monkey.rhs, monkey});
    }
  } else if (!memory.contains(monkey.name) && memory.contains(monkey.lhs) &&
             memory.contains(monkey.rhs)) {
    UpdateState(ValMonkey{monkey.name, Compute(memory, monkey)}, triggers,
                memory);
  } else if (memory.contains(monkey.name) && !memory.contains(monkey.lhs) &&
             memory.contains(monkey.rhs)) {
    UpdateState(ValMonkey{monkey.lhs, ComputeLhs(memory, monkey)}, triggers,
                memory);
  } else if (memory.contains(monkey.name) && memory.contains(monkey.lhs) &&
             !memory.contains(monkey.rhs)) {
    UpdateState(ValMonkey{monkey.rhs, ComputeRhs(memory, monkey)}, triggers,
                memory);
  } else if (add_triggers) {
    triggers.insert({monkey.name, monkey});
    triggers.insert({monkey.lhs, monkey});
    triggers.insert({monkey.rhs, monkey});
  }
}

void UpdateState(const Monkey& monkey, Triggers& triggers, Memory& memory) {
  switch (monkey.index()) {
    case 0: {  // ValMonkey
      const ValMonkey& val_monkey = std::get<ValMonkey>(monkey);
      memory.insert({val_monkey.name, val_monkey.value});
      const auto& triggered_monkeys = triggers.equal_range(val_monkey.name);
      for (auto it = triggered_monkeys.first; it != triggered_monkeys.second;
           ++it) {
        const MathMonkey& triggered_monkey = it->second;
        Propagate(triggered_monkey, triggers, memory, false);
      }
      break;
    }
    case 1: {  // MathMonkey
      const MathMonkey& math_monkey = std::get<MathMonkey>(monkey);
      Propagate(math_monkey, triggers, memory, true);
      break;
    }
    default:
      std::cerr << "Invalid monkey\n";
      exit(1);
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  Memory memory = {};
  Triggers triggers = {};
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    if (line.substr(0, 4) == "humn") {
      continue;
    }
    Monkey monkey = Parse(line);
    UpdateState(monkey, triggers, memory);
  }
  if (!memory.contains("humn")) {
    std::cerr << "No value for humn\n";
    exit(1);
  }
  return std::to_string(memory.at("humn"));
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
