/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <iostream>
#include <istream>
#include <string>
#include <string_view>

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
  int sum{0};
  int number{0};
  while (input >> number) {
    sum += number;
  }
  output << sum;
  return {};
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <sstream>

namespace {

void test_succeeds(std::string_view in, std::string_view out);

} // namespace

TEST_CASE("sums the numbers") { test_succeeds("1 2\n3", "6"); }

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
