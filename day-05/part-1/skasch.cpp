#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

std::pair<int, std::vector<std::vector<char>>> ParseInitialState(
    const std::string& input) {
  std::vector<int> offsets = {0};
  int countStacks = 0;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    offsets.push_back(offsets.back() + line.size() + 1);
    if (line.at(1) == '1') {
      countStacks = (line.size() + 1) / 4;
      break;
    }
  }

  std::vector<std::vector<char>> stacks(countStacks);
  for (int stackPosition = offsets.size() - 3; stackPosition >= 0;
       --stackPosition) {
    for (int stackIndex = 0; stackIndex < countStacks; ++stackIndex) {
      if (input.at(offsets.at(stackPosition) + 4 * stackIndex + 1) == ' ') {
        continue;
      }
      stacks.at(stackIndex)
          .push_back(input.at(offsets.at(stackPosition) + 4 * stackIndex + 1));
    }
  }

  return {offsets.back() + 1, stacks};
}

std::tuple<int, int, int> ParseInstruction(const std::string& instruction) {
  return {std::stoi(instruction.substr(5, instruction.size() - 17)),
          std::stoi(instruction.substr(instruction.size() - 6, 1)) - 1,
          std::stoi(instruction.substr(instruction.size() - 1, 1)) - 1};
}

std::string Run(const std::string& input) {
  auto [offset, stacks] = ParseInitialState(input);
  std::istringstream iss(input.substr(offset));
  for (std::string line; std::getline(iss, line);) {
    auto [crateCount, fromIndex, toIndex] = ParseInstruction(line);
    for (int crateIndex = 0; crateIndex < crateCount; ++crateIndex) {
      stacks.at(toIndex).push_back(stacks.at(fromIndex).back());
      stacks.at(fromIndex).pop_back();
    }
  }
  std::string result = "";
  for (const auto& stack : stacks) {
    result.push_back(stack.back());
  }
  return result;
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
