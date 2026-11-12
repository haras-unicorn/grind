/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <algorithm>
#include <iostream>
#include <istream>
#include <optional>
#include <queue>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

namespace {

std::optional<std::string> solve(std::istream &input, std::ostream &output);

} // namespace

#ifndef DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
int main() {
  const auto error = solve(std::cin, std::cout);
  if (error) {
    std::cerr << error.value() << '\n';
    return 1;
  }
  return 0;
}
#endif

namespace {

std::optional<std::string> solve(std::istream &input, std::ostream &output) {
  size_t count{0};
  if (!(input >> count)) {
    return {"Count of lists is undefined."};
  }
  auto lists = std::vector<std::vector<int>>(count);

  size_t total{0};

  {
    size_t current_list{0};
    std::string line;
    while (std::getline(input, line)) {
      std::istringstream line_stream{std::move(line)};

      size_t length{0};
      if (!(line_stream >> length)) {
        continue;
      }
      total += length;
      auto list = std::vector<int>(length);

      {
        int number{0};
        size_t current_number{0};
        while (line_stream >> number) {
          list[current_number++] = number;
        }
      }

      lists[current_list++] = list;
    }
  }

  typedef std::pair<std::vector<int>::iterator, std::vector<int>::iterator>
      iter_pair;
  auto iters_vector = std::vector<iter_pair>();
  for (auto current = lists.begin(); current < lists.end(); current++) {
    if (current->begin() < current->end()) {
      iters_vector.push_back(std::pair(current->begin(), current->end()));
    }
  }
  auto iters = std::priority_queue(iters_vector.begin(), iters_vector.end(),
                                   [](const iter_pair &a, const iter_pair &b) {
                                     return *a.first > *b.first;
                                   });

  auto list = std::vector<int>(total);
  for (size_t current = 0; current < total; current++) {
    auto min = iters.top();
    iters.pop();
    list[current] = *min.first;
    min.first++;
    if (min.first < min.second) {
      iters.push(min);
    }
  }

  for (auto iter = list.begin(); iter < list.end(); iter++) {
    output << *iter;
    if (iter != list.end() - 1) {
      output << " ";
    }
  }

  return {};
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <random>
#include <sstream>

namespace {

void test_succeeds(std::string_view in, std::string_view out);

} // namespace

TEST_CASE("passes example 1") {
  test_succeeds(R"(
                3
                3 1 4 7
                2 2 5
                3 3 6 9
                )",
                "1 2 3 4 5 6 7 9");
}

TEST_CASE("passes example 2") {
  test_succeeds(R"(
                2
                0
                1 -5
                )",
                "-5");
}

TEST_CASE("passes example 3") {
  test_succeeds(R"(
                2
                2 1 1
                2 1 2
                )",
                "1 1 1 2");
}

TEST_CASE("merges a single list") { test_succeeds("1\n3 5 6 7", "5 6 7"); }

TEST_CASE("prints nothing for a single empty list") {
  test_succeeds("1\n0", "");
}

TEST_CASE("prints nothing when every list is empty") {
  test_succeeds("3\n0\n0\n0", "");
}

TEST_CASE("skips an empty list at the end") {
  test_succeeds("2\n2 1 2\n0", "1 2");
}

TEST_CASE("skips an empty list in the middle") {
  test_succeeds("3\n1 5\n0\n1 3", "3 5");
}

TEST_CASE("keeps equal numbers from different lists") {
  test_succeeds("3\n2 7 7\n2 7 7\n1 7", "7 7 7 7 7");
}

TEST_CASE("handles zero, negative and extreme numbers") {
  test_succeeds("3\n2 -1000000000 5\n2 -3 1000000000\n1 0",
                "-1000000000 -3 0 5 1000000000");
}

TEST_CASE("merges lists of very different lengths") {
  test_succeeds("3\n1 100\n5 1 2 3 4 5\n2 50 60", "1 2 3 4 5 50 60 100");
}

TEST_CASE("takes from a later list while the first holds larger numbers") {
  test_succeeds("2\n2 8 9\n2 1 2", "1 2 8 9");
}

TEST_CASE("alternates between two lists") {
  test_succeeds("2\n3 1 3 5\n3 2 4 6", "1 2 3 4 5 6");
}

TEST_CASE("ignores extra spaces and blank lines") {
  test_succeeds("2\n\n  2   1 3  \n\n 1 2 \n\n", "1 2 3");
}

TEST_CASE("agrees with sorting everything on random lists") {
  std::mt19937 random{4321};
  const auto pick = [&](size_t bound) {
    return static_cast<size_t>(random()) % bound;
  };

  for (int round = 0; round < 500; round++) {
    const auto count = pick(6) + 1;
    std::string in = std::to_string(count) + "\n";
    std::vector<int> all;
    for (size_t list = 0; list < count; list++) {
      std::vector<int> numbers(pick(7));
      for (auto &number : numbers) {
        number = static_cast<int>(pick(41)) - 20;
      }
      std::ranges::sort(numbers);
      all.insert(all.end(), numbers.begin(), numbers.end());

      in += std::to_string(numbers.size());
      for (const auto number : numbers) {
        in += " " + std::to_string(number);
      }
      in += "\n";
    }
    std::ranges::sort(all);

    std::string out;
    for (size_t index = 0; index < all.size(); index++) {
      out += (index == 0 ? "" : " ") + std::to_string(all[index]);
    }
    INFO(in);
    test_succeeds(in, out);
  }
}

TEST_CASE("merges one list of 200000 numbers") {
  constexpr size_t size = 200000;
  std::string in = "1\n" + std::to_string(size);
  std::string out;
  for (size_t number = 0; number < size; number++) {
    in += " " + std::to_string(number);
    out += (number == 0 ? "" : " ") + std::to_string(number);
  }
  test_succeeds(in, out);
}

TEST_CASE("merges 1000 lists with 200 numbers each") {
  constexpr size_t lists = 1000;
  constexpr size_t per_list = 200;
  std::string in = std::to_string(lists) + "\n";
  for (size_t list = 0; list < lists; list++) {
    in += std::to_string(per_list);
    for (size_t index = 0; index < per_list; index++) {
      in += " " + std::to_string(list + index * lists);
    }
    in += "\n";
  }

  std::string out;
  for (size_t number = 0; number < lists * per_list; number++) {
    out += (number == 0 ? "" : " ") + std::to_string(number);
  }
  test_succeeds(in, out);
}

TEST_CASE("merges 1000 lists of which only one has numbers") {
  std::string in = "1000\n";
  for (size_t list = 0; list < 999; list++) {
    in += "0\n";
  }
  in += "3 -1 0 1\n";
  test_succeeds(in, "-1 0 1");
}

namespace {

void test_succeeds(std::string_view in, std::string_view out) {
  std::istringstream input{in.data()};
  std::ostringstream output{};
  const auto answer = solve(input, output);
  CHECK_MESSAGE(!answer, answer.value());
  CHECK(output.str() == out);
}

} // namespace
#endif
