/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <charconv>
#include <cstdint>
#include <expected>
#include <iostream>
#include <istream>
#include <string>
#include <string_view>
#include <system_error>

namespace {

enum class Error { NotANumber, Overflow };

std::expected<std::int64_t, Error> solve(std::istream &input);

std::expected<std::int64_t, Error> parse_number(std::string_view word);

std::string_view describe(Error error);

} // namespace

// NOTE: the test binary gets its main from doctest, so the tests can sit at the
// bottom of this file and it stays one source file to submit. Only that binary
// defines DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN - the editor sees GRIND_TEST with
// both the main and the tests
#ifndef DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
int main() {
  const auto answer = solve(std::cin);
  if (!answer) {
    std::cerr << describe(answer.error()) << '\n';
    return 1;
  }

  std::cout << *answer << '\n';
  return 0;
}
#endif

namespace {

std::expected<std::int64_t, Error> solve(std::istream &input) {
  std::int64_t sum = 0;
  std::string word;
  while (input >> word) {
    const auto number = parse_number(word);
    if (!number) {
      return std::unexpected(number.error());
    }
    if (__builtin_add_overflow(sum, *number, &sum)) {
      return std::unexpected(Error::Overflow);
    }
  }
  return sum;
}

std::expected<std::int64_t, Error> parse_number(std::string_view word) {
  std::int64_t number = 0;
  const auto *const end = word.data() + word.size();
  const auto [parsed, error] = std::from_chars(word.data(), end, number);
  if (error != std::errc{} || parsed != end) {
    return std::unexpected(Error::NotANumber);
  }
  return number;
}

std::string_view describe(Error error) {
  switch (error) {
  case Error::NotANumber:
    return "the input has a word that is not a number";
  case Error::Overflow:
    return "the sum does not fit in 64 bits";
  }
  return "unknown error";
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <limits>
#include <sstream>

TEST_CASE("sums the numbers") {
  std::istringstream input{"1 2\n3\n"};
  CHECK(solve(input) == 6);
}

TEST_CASE("rejects a word that is not a number") {
  std::istringstream input{"one"};
  CHECK(solve(input) == std::unexpected(Error::NotANumber));
}

TEST_CASE("reports an overflow") {
  std::istringstream input{
      std::to_string(std::numeric_limits<std::int64_t>::max()) + " 1"};
  CHECK(solve(input) == std::unexpected(Error::Overflow));
}

TEST_CASE("describes every error") {
  CHECK_FALSE(describe(Error::NotANumber).empty());
  CHECK_FALSE(describe(Error::Overflow).empty());
}
#endif
