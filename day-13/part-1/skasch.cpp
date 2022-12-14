#include <chrono>
#include <compare>
#include <iostream>
#include <memory>
#include <span>
#include <sstream>
#include <string>
#include <variant>
#include <vector>

struct Packet {
  std::variant<int, std::vector<Packet>> data;
};

void Display(const Packet& packet) {
  switch (packet.data.index()) {
    case 0: {  // int
      std::cerr << std::get<int>(packet.data);
      break;
    }
    case 1: {  // std::vector<Packet>
      std::cerr << "[ ";
      for (const auto& packet : std::get<std::vector<Packet>>(packet.data)) {
        Display(packet);
        std::cerr << ' ';
      }
      std::cerr << ']';
      break;
    }
    default:
      std::cerr << "Invalid variant for packet";
      exit(1);
  }
}

Packet ParseLineAt(const std::string& line, int& position) {
  if (position >= line.size()) {
    std::cerr << "Error while parsing " << line << ": reached end of string";
    exit(1);
  }
  if (line.at(position) != '[') {
    int npos = 1;
    while (
        !(line.at(position + npos) == ',' || line.at(position + npos) == ']')) {
      ++npos;
    }
    position += npos;
    return Packet{std::stoi(line.substr(position - npos, npos))};
  }
  std::vector<Packet> content;
  ++position;
  while (line.at(position) != ']') {
    if (line.at(position) == ',') {
      ++position;
    }
    content.emplace_back(ParseLineAt(line, position));
  }
  ++position;
  return Packet{content};
}

std::weak_ordering operator<=>(const Packet& lhs, const Packet& rhs) {
  if (lhs.data.index() == 0 && rhs.data.index() == 0) {
    return std::get<int>(lhs.data) <=> std::get<int>(rhs.data);
  }
  if (lhs.data.index() == 1 && rhs.data.index() == 1) {
    std::vector<Packet> lhs_data = std::get<std::vector<Packet>>(lhs.data);
    std::vector<Packet> rhs_data = std::get<std::vector<Packet>>(rhs.data);
    for (int index = 0; index < lhs_data.size() && index < rhs_data.size();
         ++index) {
      std::weak_ordering result = lhs_data.at(index) <=> rhs_data.at(index);
      if (result != std::weak_ordering::equivalent) {
        return result;
      }
    }
    return lhs_data.size() <=> rhs_data.size();
  }
  if (lhs.data.index() == 0) {
    return Packet{std::vector<Packet>{Packet{std::get<int>(lhs.data)}}} <=> rhs;
  } else {
    return lhs <=> Packet{std::vector<Packet>{Packet{std::get<int>(rhs.data)}}};
  }
}

Packet ParseLine(const std::string& line) {
  int position = 0;
  return ParseLineAt(line, position);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int index = 1;
  int result = 0;
  for (std::string line; std::getline(iss, line);) {
    if (line.size() == 0) {
      continue;
    }
    Packet first_packet = ParseLine(line);
    std::getline(iss, line);
    Packet second_packet = ParseLine(line);
    if (first_packet <=> second_packet == std::weak_ordering::less) {
      result += index;
    }
    ++index;
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
