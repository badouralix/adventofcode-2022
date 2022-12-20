#include <algorithm>
#include <chrono>
#include <iostream>
#include <ostream>
#include <regex>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

static constexpr int kMaxTime = 32;
static constexpr int kMaxBlueprint = 3;

static const std::regex kBlueprintRegex(
    "Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot "
    "costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) "
    "clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.");

struct Blueprint {
  int id;
  int ore_robot_ore_cost;
  int clay_robot_ore_cost;
  int obsidian_robot_ore_cost;
  int obsidian_robot_clay_cost;
  int geode_robot_ore_cost;
  int geode_robot_obsidian_cost;
};

std::ostream& operator<<(std::ostream& out, const Blueprint& blueprint) {
  out << "BP #" << blueprint.id << ": OrR[" << blueprint.ore_robot_ore_cost
      << "Or] ClR[" << blueprint.clay_robot_ore_cost << "Or] ObR["
      << blueprint.obsidian_robot_ore_cost << "Or "
      << blueprint.obsidian_robot_clay_cost << "Cl] GeR["
      << blueprint.geode_robot_ore_cost << "Or "
      << blueprint.geode_robot_obsidian_cost << "Ob]";
  return out;
}

using State = std::tuple<int, int, int, int, int, int, int, int>;

int GetTime(const State& state) { return std::get<0>(state); }
int GetGeode(const State& state) { return std::get<1>(state); }
int GetOreRobots(const State& state) { return std::get<2>(state); }
int GetClayRobots(const State& state) { return std::get<3>(state); }
int GetObsidianRobots(const State& state) { return std::get<4>(state); }
int GetOre(const State& state) { return std::get<5>(state); }
int GetClay(const State& state) { return std::get<6>(state); }
int GetObsidian(const State& state) { return std::get<7>(state); }

std::ostream& operator<<(std::ostream& out, const State& state) {
  out << "Time: " << GetTime(state) << " | Robots: " << GetOreRobots(state)
      << " R " << GetClayRobots(state) << " C " << GetObsidianRobots(state)
      << " O | Resources: " << GetOre(state) << " R " << GetClay(state) << " C "
      << GetObsidian(state) << " O | Max geodes: " << GetGeode(state);
  return out;
}

int EstimateMaxGeodes(const Blueprint& blueprint, const State& state) {
  // Consider that we build 1 obsidian robot per minute, AND that we can build
  // geode robots concurrently considering only the obsidian cost.
  int obsidian = GetObsidian(state);
  int obsidian_robots = GetObsidianRobots(state);
  int geode = 0;
  for (int time = GetTime(state); time < kMaxTime; ++time) {
    if (obsidian >= blueprint.geode_robot_obsidian_cost) {
      geode += kMaxTime - time - 1;
      obsidian -= blueprint.geode_robot_obsidian_cost;
    }
    obsidian += obsidian_robots;
    ++obsidian_robots;
  }
  return GetGeode(state) + geode;
}

void PushBack(const Blueprint& blueprint, const State& state, int best_result,
              const std::set<State>& visited, std::vector<State>& stack,
              int ore_robot, int clay_robot, int obsidian_robot,
              int geode_robot) {
  const State next_state = {
      GetTime(state) + 1,
      GetGeode(state) + (kMaxTime - GetTime(state) - 1) * geode_robot,
      GetOreRobots(state) + ore_robot,
      GetClayRobots(state) + clay_robot,
      GetObsidianRobots(state) + obsidian_robot,
      GetOre(state) + GetOreRobots(state) -
          ore_robot * blueprint.ore_robot_ore_cost -
          clay_robot * blueprint.clay_robot_ore_cost -
          obsidian_robot * blueprint.obsidian_robot_ore_cost -
          geode_robot * blueprint.geode_robot_ore_cost,
      GetClay(state) + GetClayRobots(state) -
          obsidian_robot * blueprint.obsidian_robot_clay_cost,
      GetObsidian(state) + GetObsidianRobots(state) -
          geode_robot * blueprint.geode_robot_obsidian_cost,
  };
  if (!visited.contains(next_state) &&
      EstimateMaxGeodes(blueprint, next_state) >= best_result) {
    stack.emplace_back(std::move(next_state));
  }
}

int Dfs(const Blueprint& blueprint) {
  int result = 0;
  // Thanks thomren for the idea!
  const int max_ore_cost = std::max(
      {blueprint.ore_robot_ore_cost, blueprint.clay_robot_ore_cost,
       blueprint.obsidian_robot_ore_cost, blueprint.geode_robot_ore_cost});
  std::vector<State> stack = {{0, 0, 1, 0, 0, 0, 0, 0}};
  std::set<State> visited;
  while (!stack.empty()) {
    State state = stack.back();
    stack.pop_back();
    if (visited.contains(state)) {
      continue;
    }
    visited.insert(state);
    if (EstimateMaxGeodes(blueprint, state) <= result) {
      continue;
    }
    if (GetTime(state) == kMaxTime) {
      result = std::max(result, GetGeode(state));
      continue;
    }
    PushBack(blueprint, state, result, visited, stack, 0, 0, 0, 0);
    if (GetOreRobots(state) < max_ore_cost &&
        GetOre(state) >= blueprint.ore_robot_ore_cost) {
      PushBack(blueprint, state, result, visited, stack, 1, 0, 0, 0);
    }
    if (GetOre(state) >= blueprint.clay_robot_ore_cost) {
      PushBack(blueprint, state, result, visited, stack, 0, 1, 0, 0);
    }
    if (GetOre(state) >= blueprint.obsidian_robot_ore_cost &&
        GetClay(state) >= blueprint.obsidian_robot_clay_cost) {
      PushBack(blueprint, state, result, visited, stack, 0, 0, 1, 0);
    }
    if (GetOre(state) >= blueprint.geode_robot_ore_cost &&
        GetObsidian(state) >= blueprint.geode_robot_obsidian_cost) {
      PushBack(blueprint, state, result, visited, stack, 0, 0, 0, 1);
    }
  }
  return result;
}

Blueprint ParseBlueprint(const std::string& line) {
  std::smatch blueprint_match;
  if (std::regex_match(line, blueprint_match, kBlueprintRegex)) {
    return Blueprint{
        std::stoi(blueprint_match[1].str()),
        std::stoi(blueprint_match[2].str()),
        std::stoi(blueprint_match[3].str()),
        std::stoi(blueprint_match[4].str()),
        std::stoi(blueprint_match[5].str()),
        std::stoi(blueprint_match[6].str()),
        std::stoi(blueprint_match[7].str()),
    };
  }
  exit(1);
}

std::string Run(const std::string& input) {
  // Your code goes here
  int result = 1;
  std::istringstream iss(input);
  std::string line;
  for (int row = 0; row < kMaxBlueprint; ++row) {
    std::getline(iss, line);
    Blueprint blueprint = ParseBlueprint(line);
    result *= Dfs(blueprint);
  }
  return std::to_string(result);
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
