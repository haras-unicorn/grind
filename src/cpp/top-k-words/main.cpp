/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <algorithm>
#include <iostream>
#include <istream>
#include <optional>
#include <ranges>
#include <string>
#include <string_view>
#include <unordered_map>
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
  size_t k{0};
  input >> k;

  size_t n{0};
  input >> n;

  std::unordered_map<std::string, size_t> words{};
  {
    std::string word;
    while (input >> word) {
      words[word]++;
    }
  }

  std::vector<std::pair<std::string, size_t>> sorted(words.begin(),
                                                     words.end());
  std::ranges::sort(sorted, [](auto &&a, auto &&b) {
    return a.second == b.second ? a.first < b.first : a.second > b.second;
  });

  for (const auto &[word, count] : sorted | std::ranges::views::take(k)) {
    output << word << " " << count << "\n";
  }

  return {};
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <sstream>

namespace {

void test_succeeds(std::string_view in, std::string_view out);

// four letters, so every number below 26^4 is a word, and the words sort the
// same way as the numbers
std::string word_of(size_t number);

} // namespace

TEST_CASE("prints top k words") {
  test_succeeds("2 6\nthe day is sunny the the", "the 3\nday 1\n");
}

TEST_CASE("ranks by count first") {
  test_succeeds("3 7\ni love leetcode i love coding i",
                "i 3\nlove 2\ncoding 1\n");
}

TEST_CASE("prints only the distinct words when there are fewer than k") {
  test_succeeds("5 3\na b a", "a 2\nb 1\n");
}

TEST_CASE("breaks ties alphabetically, not by first appearance") {
  test_succeeds("3 5\nzebra apple mango apple zebra",
                "apple 2\nzebra 2\nmango 1\n");
}

TEST_CASE("breaks ties by letters, not by length") {
  test_succeeds("2 2\nb ab", "ab 1\nb 1\n");
}

TEST_CASE("treats a word that starts another word as a different word") {
  test_succeeds("2 4\nab a ab abc", "ab 2\na 1\n");
}

TEST_CASE("picks the first word alphabetically when everything ties") {
  test_succeeds("1 3\nc b a", "a 1\n");
}

TEST_CASE("prints every word when k is the number of words") {
  test_succeeds("4 4\nd b a c", "a 1\nb 1\nc 1\nd 1\n");
}

TEST_CASE("counts one word repeated") {
  test_succeeds("1 5\nx x x x x", "x 5\n");
}

TEST_CASE("reads words across lines and extra spaces") {
  test_succeeds("2 5\n  b\n\n a   b\n a b  \n", "b 3\na 2\n");
}

TEST_CASE("sorts many distinct words fed in reverse order") {
  constexpr size_t count = 200000;
  std::string in = "200000 200000\n";
  std::string out;
  for (size_t number = 0; number < count; number++) {
    in += word_of(count - 1 - number) + "\n";
    out += word_of(number) + " 1\n";
  }
  test_succeeds(in, out);
}

TEST_CASE("breaks ties alphabetically on a large input") {
  constexpr size_t count = 200000;
  std::string in = "26 200000\n";
  for (size_t number = 0; number < count; number++) {
    in += static_cast<char>('a' + number % 26);
    in += ' ';
  }

  // NOTE: 200000 = 26 * 7692 + 8, so the first 8 letters appear once more
  std::string out;
  for (size_t letter = 0; letter < 26; letter++) {
    const auto times = count / 26 + static_cast<size_t>(letter < count % 26);
    out += static_cast<char>('a' + letter);
    out += ' ' + std::to_string(times) + '\n';
  }
  test_succeeds(in, out);
}

namespace {

std::string word_of(size_t number) {
  std::string word(4, 'a');
  for (auto position = word.size(); position > 0; position--) {
    word[position - 1] = static_cast<char>('a' + number % 26);
    number /= 26;
  }
  return word;
}

void test_succeeds(std::string_view in, std::string_view out) {
  std::istringstream input{in.data()};
  std::ostringstream output{};
  const auto answer = solve(input, output);
  CHECK_MESSAGE(!answer, answer.value());
  CHECK(output.str() == out);
}

} // namespace
#endif
