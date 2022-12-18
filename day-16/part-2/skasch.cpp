#include <array>
#include <bitset>
#include <chrono>
#include <compare>
#include <iostream>
#include <map>
#include <ostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

int kTime = 26;

struct Valve {
  std::string name;
  int flow_rate;
  std::vector<std::string> tunnels;
};

struct State {
  int pos1;
  int pos2;
  int valves;
  int remaining_pressure;
};

std::strong_ordering operator<=>(const State& lhs, const State& rhs) {
  return std::tuple<int, int, int>(lhs.pos1, lhs.pos2, lhs.valves) <=>
         std::tuple<int, int, int>(rhs.pos1, rhs.pos2, rhs.valves);
}

std::ostream& operator<<(std::ostream& out, const State& state) {
  out << "State{" << state.pos1 << ',' << state.pos2 << ','
      << std::bitset<16>(state.valves) << '}';
  return out;
}

class Network {
 public:
  Network(const std::string& input)
      : valves_{}, index_{}, mask_index_{}, total_pressure_{} {
    std::istringstream iss(input);
    for (std::string line; std::getline(iss, line);) {
      std::string name = line.substr(6, 2);
      int semicolon_pos = line.find(';', 24);
      int flow_rate = std::stoi(line.substr(23, semicolon_pos - 23));
      int tunnel_pos = line.find(' ', semicolon_pos + 23) + 1;
      std::vector<std::string> tunnels;
      while (tunnel_pos < line.size()) {
        tunnels.emplace_back(line.substr(tunnel_pos, 2));
        tunnel_pos += 4;
      }
      index_.insert({name, valves_.size()});
      if (flow_rate > 0) {
        mask_index_.insert({valves_.size(), mask_index_.size()});
        total_pressure_ += flow_rate;
      }
      valves_.emplace_back(Valve{name, flow_rate, tunnels});
    }
  }

  void Display(std::ostream& out) const {
    for (const Valve& valve : valves_) {
      out << valve.name << '[' << valve.flow_rate << "] => { ";
      for (std::string_view tunnel : valve.tunnels) {
        out << tunnel << ' ';
      }
      out << "}\n";
    }
    for (const auto& [name, index] : index_) {
      out << name << ':' << index << ' ';
    }
    out << '\n';
  }

  int GetMaskSize() const { return 1 << mask_index_.size(); }
  int GetMaskIndex(int position) const { return mask_index_.at(position); }
  int GetPosition(const std::string& name) const { return index_.at(name); }
  int GetTotalPressure() const { return total_pressure_; }

  const Valve& GetValve(int position) const { return valves_.at(position); }
  std::vector<int> GetNeighbors(int position) const {
    std::vector<int> neighbors;
    for (const std::string& tunnel : GetValve(position).tunnels) {
      neighbors.push_back(index_.at(tunnel));
    }
    return neighbors;
  }

 private:
  std::vector<Valve> valves_;
  std::map<std::string, int> index_;
  std::map<int, int> mask_index_;
  int total_pressure_;
};

int BestReachableScore(const State& state, int score, int time) {
  return score + state.remaining_pressure * time;
}

void UpdateNextStates(const State& next_state,
                      std::map<State, int>& next_states, int score) {
  if (next_states.contains(next_state)) {
    next_states.at(next_state) = std::max(next_states.at(next_state), score);
  } else {
    next_states.insert({next_state, score});
  }
}

std::ostream& operator<<(std::ostream& out,
                         const std::map<State, int>& states) {
  out << "{ ";
  for (const auto& [state, score] : states) {
    out << state << ": " << score << ' ';
  }
  out << '}';
  return out;
}

std::string Run(const std::string& input) {
  const Network network(input);
  // network.Display(std::cerr);
  std::map<State, int> reached_states = {
      {{network.GetPosition("AA"), network.GetPosition("AA"), 0,
        network.GetTotalPressure()},
       0}};
  int result = 0;
  for (int time = kTime - 1; time >= 0; --time) {
    // std::cerr << "Time left : " << time << '\n';
    // std::cerr << "Best result: " << result << '\n';
    // std::cerr << "Number of states: " << reached_states.size() << '\n';
    std::map<State, int> mid_states;
    for (const auto& [state, score] : reached_states) {
      if (BestReachableScore(state, score, time) <= result) {
        continue;
      }
      int flow_rate1 = network.GetValve(state.pos1).flow_rate;
      if (flow_rate1 > 0) {
        int valve_mask = 1 << network.GetMaskIndex(state.pos1);
        if ((state.valves & valve_mask) == 0) {
          State next_state = {state.pos1, state.pos2, state.valves | valve_mask,
                              state.remaining_pressure - flow_rate1};
          if (BestReachableScore(next_state, score + flow_rate1 * time,
                                 time - 1) > result) {
            UpdateNextStates(next_state, mid_states, score + flow_rate1 * time);
            result = std::max(result, score + flow_rate1 * time);
          }
        }
      }
      for (int neighbor : network.GetNeighbors(state.pos1)) {
        State next_state = {neighbor, state.pos2, state.valves,
                            state.remaining_pressure};
        if (BestReachableScore(next_state, score, time - 1) > result) {
          UpdateNextStates(next_state, mid_states, score);
        }
      }
    }
    std::map<State, int> next_states;
    for (const auto& [state, score] : mid_states) {
      int flow_rate2 = network.GetValve(state.pos2).flow_rate;
      if (flow_rate2 > 0) {
        int valve_mask = 1 << network.GetMaskIndex(state.pos2);
        if ((state.valves & valve_mask) == 0) {
          State next_state = {state.pos1, state.pos2, state.valves | valve_mask,
                              state.remaining_pressure - flow_rate2};
          if (BestReachableScore(next_state, score + flow_rate2 * time,
                                 time - 1) > result) {
            UpdateNextStates(next_state, next_states,
                             score + flow_rate2 * time);
            result = std::max(result, score + flow_rate2 * time);
          }
        }
      }
      for (int pos2 : network.GetNeighbors(state.pos2)) {
        int pos1 = state.pos1;
        if (pos1 > pos2) {
          std::swap(pos1, pos2);
        }
        State next_state = {pos1, pos2, state.valves, state.remaining_pressure};
        if (BestReachableScore(next_state, score, time - 1) > result) {
          UpdateNextStates(next_state, next_states, score);
        }
      }
    }
    std::swap(next_states, reached_states);
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
