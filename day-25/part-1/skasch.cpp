#include <chrono>
#include <cstdint>
#include <iostream>
#include <map>
#include <span>
#include <sstream>
#include <string>
#include <vector>

inline constexpr int kSnafuBase = 5;

static const std::vector<char> kSnafuDigits = {'=', '-', '0', '1', '2'};

static const std::map<char, int> kSnafuToDigit = {
    {'=', -2}, {'-', -1}, {'0', 0}, {'1', 1}, {'2', 2},
};
static const std::map<int, char> kDigitToSnafu = {
    {-2, '='}, {-1, '-'}, {0, '0'}, {1, '1'}, {2, '2'},
};

std::int64_t ToDecimal(const std::string& snafu) {
  std::int64_t result = 0;
  std::int64_t base = 1;
  for (int index = snafu.size() - 1; index >= 0; --index) {
    result += base * kSnafuToDigit.at(snafu.at(index));
    base *= kSnafuBase;
  }
  return result;
}

std::string ToSnafu(std::int64_t decimal) {
  std::int64_t base = 1;
  while (kSnafuBase * base < 2 * decimal) {
    base *= kSnafuBase;
  }
  if (base == 1) {
    return {kDigitToSnafu.at(decimal)};
  }
  std::string snafu_digits;
  std::int64_t digit = ((2 * decimal) / base + 1) / 2;
  snafu_digits.push_back(kDigitToSnafu.at(digit));
  decimal -= digit * base;
  while (base > 1) {
    base /= kSnafuBase;
    if (base == 1) {
      snafu_digits.push_back(kDigitToSnafu.at(decimal));
      break;
    }
    std::int64_t digit = ((2 * decimal) / base + (decimal > 0 ? 1 : -1)) / 2;
    snafu_digits.push_back(kDigitToSnafu.at(digit));
    decimal -= digit * base;
  }
  std::string snafu;
  for (const char& c : snafu_digits) {
    snafu.push_back(c);
  }
  return snafu;
}

std::string Run(const std::string& input) {
  // // Your code goes here
  // std::multimap<int, std::string> number_to_snafu;
  // for (const auto& c0 : kSnafuDigits) {
  //   int d0 = kSnafuToDigit.at(c0);
  //   for (const auto& c1 : kSnafuDigits) {
  //     int d1 = kSnafuToDigit.at(c1);
  //     for (const auto& c2 : kSnafuDigits) {
  //       int d2 = kSnafuToDigit.at(c2);
  //       for (const auto& c3 : kSnafuDigits) {
  //         int d3 = kSnafuToDigit.at(c3);
  //         number_to_snafu.insert({d0 + kSnafuBase * d1 +
  //                                     kSnafuBase * kSnafuBase * d2 +
  //                                     kSnafuBase * kSnafuBase * kSnafuBase *
  //                                     d3,
  //                                 {c3, c2, c1, c0}});
  //       }
  //     }
  //   }
  // }
  // for (const auto& [number, snafu] : number_to_snafu) {
  //   if (number < 0) {
  //     continue;
  //   }
  //   std::cerr << number << ": " << snafu;
  //   std::string snafu2 = ToSnafu(number);
  //   std::cerr << " =? " << snafu2 << '\n';
  // }
  std::istringstream iss(input);
  std::int64_t total_fuel = 0;
  for (std::string line; std::getline(iss, line);) {
    total_fuel += ToDecimal(line);
  }
  return ToSnafu(total_fuel);
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
