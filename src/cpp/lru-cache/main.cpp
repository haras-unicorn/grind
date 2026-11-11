/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <cstddef>
#include <expected>
#include <format>
#include <iostream>
#include <istream>
#include <list>
#include <optional>
#include <ostream>
#include <string>
#include <tuple>
#include <unordered_map>

namespace {

class LruCache {
  size_t max_size;
  size_t size;
  std::list<std::tuple<int, int>> lru;
  std::unordered_map<int, std::list<std::tuple<int, int>>::iterator> cache;

public:
  LruCache(size_t size);

  void put(int key, int value);
  std::optional<int> get(int key);
};

std::expected<LruCache, std::string> solve(std::istream &input,
                                           std::ostream &output);

} // namespace

// NOTE: the test binary gets its main from doctest, so the tests can sit at the
// bottom of this file and it stays one source file to submit. Only that binary
// defines DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN - the editor sees GRIND_TEST with
// both the main and the tests
#ifndef DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
int main() {
  const auto answer = solve(std::cin, std::cout);
  if (!answer) {
    std::cerr << answer.error() << '\n';
    return 1;
  }

  return 0;
}
#endif

namespace {

LruCache::LruCache(size_t _max_size)
    : max_size{_max_size}, size{0}, lru{0}, cache{0} {}

std::optional<int> LruCache::get(int key) {
  if (!cache.contains(key)) {
    return {};
  }

  const auto iter = cache[key];
  const auto key_value = *iter;
  lru.erase(iter);
  lru.push_front(key_value);
  cache[key] = lru.begin();
  return std::get<1>(key_value);
}

void LruCache::put(int key, int value) {
  const auto is_full = size == max_size;
  const auto has_key = cache.contains(key);

  if (is_full && !has_key) {
    const auto key_value = lru.back();
    lru.pop_back();
    cache.erase(std::get<0>(key_value));
    lru.push_front({key, value});
    cache[key] = lru.begin();
  } else if (has_key) {
    const auto iter = cache[key];
    lru.erase(iter);
    lru.push_front({key, value});
    cache[key] = lru.begin();
  } else {
    lru.push_front({key, value});
    cache[key] = lru.begin();
    size++;
  }
}

std::expected<LruCache, std::string> solve(std::istream &input,
                                           std::ostream &output) {
  size_t max_size{0};
  if (!(input >> max_size)) {
    return std::unexpected("Max size is undefined.");
  }
  if (max_size == 0) {
    return std::unexpected("Max size is 0.");
  }
  LruCache cache = {max_size};

  std::string command;
  while (input >> command) {
    if (command == "PUT") {
      int key{0};
      if (!(input >> key)) {
        return std::unexpected("Put key is undefined.");
      }
      int value{0};
      if (!(input >> value)) {
        return std::unexpected("Put value is undefined.");
      }
      cache.put(key, value);
    } else if (command == "GET") {
      int key{0};
      if (!(input >> key)) {
        return std::unexpected("Get key is undefined.");
      }

      const auto value_result = cache.get(key);
      if (value_result.has_value()) {
        output << value_result.value() << "\n";
      } else {
        output << -1 << "\n";
      }
    } else {
      return std::unexpected(std::format("Unknown command: {}", command));
    }
  }

  return cache;
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <sstream>

namespace {

void test_lru(std::string_view in, std::string_view out);

void test_parse_fail(std::string_view in);

void test_parse_success(std::string_view in);

} // namespace

TEST_CASE("caches the values") { test_lru("1\nPUT 1 11\nGET 1\n", "11\n"); }

TEST_CASE("returns -1 on unknown key") {
  test_lru("1\nPUT 1 11\nGET 2\n", "-1\n");
}

TEST_CASE("evicts least recently used") {
  test_lru("1\nPUT 1 11\nGET 1\nPUT 2 12\nGET 1\n", "11\n-1\n");
}

TEST_CASE("gets twice in a row correctly") {
  test_lru("1\nPUT 1 11\nGET 1\nGET 1\n", "11\n11\n");
}

TEST_CASE("overwrites existing") {
  test_lru("1\nPUT 1 11\nPUT 1 12\nGET 1\n", "12\n");
}

TEST_CASE("overwrite key at capacity 2") {
  test_lru("2\nPUT 1 11\nGET 1\nPUT 1 12\nGET 1\nPUT 2 13\nGET 1\n",
           "11\n12\n12\n");
}

TEST_CASE("keep key at capacity 2") {
  test_lru("2\nPUT 1 11\nGET 1\nPUT 2 12\nGET 2\nPUT 2 13\nGET 1\n",
           "11\n12\n11\n");
}

TEST_CASE("get makes the key the most recently used") {
  test_lru("2\nPUT 1 11\nPUT 2 12\nGET 1\nPUT 3 13\nGET 2\nGET 1\nGET 3\n",
           "11\n-1\n11\n13\n");
}

TEST_CASE("put makes the key the most recently used") {
  test_lru("2\nPUT 1 11\nPUT 2 12\nPUT 1 21\nPUT 3 13\nGET 2\nGET 1\nGET 3\n",
           "-1\n21\n13\n");
}

TEST_CASE("misses do not change the order") {
  test_lru("2\nPUT 1 11\nPUT 2 12\nGET 3\nPUT 3 13\nGET 1\nGET 2\n",
           "-1\n-1\n12\n");
}

TEST_CASE("writes an evicted key again") {
  test_lru("1\nPUT 1 11\nPUT 2 12\nPUT 1 13\nGET 1\nGET 2\n", "13\n-1\n");
}

TEST_CASE("repeated overwrites do not use up the capacity") {
  test_lru("2\nPUT 1 1\nPUT 1 2\nPUT 1 3\nPUT 2 4\nGET 1\nGET 2\n", "3\n4\n");
}

TEST_CASE("evicts one key at a time") {
  test_lru(
      "2\nPUT 1 1\nPUT 2 2\nPUT 3 3\nPUT 4 4\nGET 1\nGET 2\nGET 3\nGET 4\n",
      "-1\n-1\n3\n4\n");
}

TEST_CASE("handles zero, negative and extreme numbers") {
  test_lru("3\nPUT 0 0\nPUT -5 -7\nPUT 2147483647 -2147483648\n"
           "GET 0\nGET -5\nGET 2147483647\n",
           "0\n-7\n-2147483648\n");
}

TEST_CASE("fails on undefined max size") { test_parse_fail(""); }

TEST_CASE("fails on max size 0") { test_parse_fail("0"); }

TEST_CASE("fails on unknown command") { test_parse_fail("1\nDELETE\n"); }

TEST_CASE("fails on undefined put key") { test_parse_fail("1\nPUT\n"); }

TEST_CASE("fails on undefined put value") { test_parse_fail("1\nPUT 1\n"); }

TEST_CASE("fails on undefined get key") { test_parse_fail("1\nGET\n"); }

TEST_CASE("fails on a key that does not fit in an int") {
  test_parse_fail("1\nPUT 99999999999 1\n");
}

TEST_CASE("fails on an extra word after a command") {
  test_parse_fail("1\nGET 1 2\n");
}

TEST_CASE("succeeds on random whitespace") {
  test_parse_success("1\n GET 1   \n \n  PUT 3   5323 \n");
}

namespace {

void test_lru(std::string_view in, std::string_view out) {
  std::istringstream input{in.data()};
  std::ostringstream output{};
  const auto answer = solve(input, output);
  CHECK_MESSAGE(!!answer, answer.error());
  CHECK(output.str() == out);
}

void test_parse_fail(std::string_view in) {
  std::istringstream input{in.data()};
  std::ostringstream output{};
  const auto answer = solve(input, output);
  CHECK(!answer);
}

void test_parse_success(std::string_view in) {
  std::istringstream input{in.data()};
  std::ostringstream output{};
  const auto answer = solve(input, output);
  CHECK_MESSAGE(!!answer, answer.error());
}

} // namespace
#endif
