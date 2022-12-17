#include <array>
#include <bitset>
#include <chrono>
#include <iostream>
#include <map>
#include <ostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

int kTime = 30;

struct Valve {
  std::string name;
  int flow_rate;
  std::vector<std::string> tunnels;
};

using State = std::pair<int, int>;

std::ostream& operator<<(std::ostream& out, const State& state) {
  out << "State{" << state.first << ',' << std::bitset<16>(state.second) << '}';
  return out;
}

class Network {
 public:
  Network(const std::string& input) : valves_{}, index_{}, mask_index_{} {
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
};

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
  std::map<State, int> reached_states = {{{network.GetPosition("AA"), 0}, 0}};
  int result = 0;
  for (int time = kTime - 1; time >= 0; --time) {
    std::map<State, int> next_states;
    for (const auto& [state, score] : reached_states) {
      int flow_rate = network.GetValve(state.first).flow_rate;
      if (flow_rate > 0) {
        int valve_mask = 1 << network.GetMaskIndex(state.first);
        if ((state.second & valve_mask) == 0) {
          State next_state = {state.first, state.second | valve_mask};
          UpdateNextStates(next_state, next_states, score + flow_rate * time);
          result = std::max(result, score + flow_rate * time);
        }
      }
      for (int neighbor : network.GetNeighbors(state.first)) {
        State next_state = {neighbor, state.second};
        UpdateNextStates(next_state, next_states, score);
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
