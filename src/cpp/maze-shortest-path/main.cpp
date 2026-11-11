/// @file
/// Reads the input from stdin and prints the answer to stdout.

#include <iostream>
#include <istream>
#include <queue>
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
  int height{0};
  input >> height;

  int width{0};
  input >> width;

  // output << height << " " << width << "\n";

  struct point {
    int height;
    int width;
  };

  std::vector<std::vector<int>> paths{};
  std::vector<std::string> lines{};
  point start{};
  point end{};
  std::string line;
  int i{0};
  while (input >> line) {
    // output << line << "\n";
    lines.push_back(line);
    paths.push_back(std::vector(static_cast<size_t>(width), -1));
    int j = 0;
    for (const auto tile : line) {
      if (tile == 'S') {
        start = {i, j};
      }
      if (tile == 'E') {
        end = {i, j};
      }
      j++;
    }
    i++;
  }

  std::queue<std::pair<point, int>> to_visit{};
  to_visit.push({start, 0});
  while (!to_visit.empty()) {
    const auto [current, length] = to_visit.front();
    to_visit.pop();

    if (current.height < 0 || current.width < 0 || current.height >= height ||
        current.width >= width) {
      continue;
    }

    const auto tile = lines[static_cast<size_t>(current.height)]
                           [static_cast<size_t>(current.width)];

    if (tile == 'S') {
      paths[static_cast<size_t>(current.height)]
           [static_cast<size_t>(current.width)] = 0;
    }

    if (tile == '#') {
      continue;
    }

    if (tile == 'E') {
      auto *path = &paths[static_cast<size_t>(current.height)]
                         [static_cast<size_t>(current.width)];
      if (*path == -1 || length < *path) {
        *path = length;
      }
      break;
    }

    if (tile == '.') {
      auto *path = &paths[static_cast<size_t>(current.height)]
                         [static_cast<size_t>(current.width)];
      if (*path == -1 || length < *path) {
        *path = length;
      } else {
        continue;
      }
    }

    to_visit.push({{current.height - 1, current.width}, length + 1});
    to_visit.push({{current.height, current.width - 1}, length + 1});
    to_visit.push({{current.height + 1, current.width}, length + 1});
    to_visit.push({{current.height, current.width + 1}, length + 1});
  }

  // for (const auto &l : paths) {
  //   for (const auto &p : l) {
  //     output << p << " ";
  //   }
  //   output << "\n";
  // }
  // output << "\n";

  output
      << paths[static_cast<size_t>(end.height)][static_cast<size_t>(end.width)]
      << "\n";

  return {};
}

} // namespace

#ifdef GRIND_TEST
#include <doctest/doctest.h>

#include <queue>
#include <random>
#include <sstream>

namespace {

void test_succeeds(std::string_view in, std::string_view out);

// a plain search level by level, to compare the solution against
int reference_distance(const std::vector<std::string> &grid);

} // namespace

TEST_CASE("finds the shortest path") {
  test_succeeds("3 4\nS..#\n.#..\n...E", "5\n");
}

TEST_CASE("prints -1 when a wall is in the way") {
  test_succeeds("2 3\nS#E\n.#.", "-1\n");
}

TEST_CASE("moves to the exit next to the start") {
  test_succeeds("1 2\nSE", "1\n");
}

TEST_CASE("moves down") { test_succeeds("2 1\nS\nE", "1\n"); }

TEST_CASE("walks along a corridor") { test_succeeds("1 5\nS...E", "4\n"); }

TEST_CASE("walks left") { test_succeeds("1 4\nE..S", "3\n"); }

TEST_CASE("goes around a wall") { test_succeeds("3 3\nS#E\n.#.\n...", "6\n"); }

TEST_CASE("prints -1 when the start is walled in") {
  test_succeeds("3 3\nS#.\n###\n..E", "-1\n");
}

TEST_CASE("crosses an open room") {
  test_succeeds("4 4\nS...\n....\n....\n...E", "6\n");
}

TEST_CASE("reaches an exit in the middle") {
  test_succeeds("3 3\nS..\n.E.\n...", "2\n");
}

TEST_CASE("goes up and to the right") {
  test_succeeds("3 3\n..E\n...\nS..", "4\n");
}

TEST_CASE("agrees with a plain search on random grids") {
  std::mt19937 random{12345};
  const auto pick = [&](size_t bound) {
    return static_cast<size_t>(random()) % bound;
  };

  for (int round = 0; round < 400; round++) {
    // NOTE: half of the grids are small, and half are big enough for long paths
    const auto side = round % 2 == 0 ? size_t{8} : size_t{40};
    const auto rows = pick(side) + 1;
    const auto cols = pick(side) + 1;
    if (rows * cols < 2) {
      continue;
    }

    std::vector<std::string> grid(rows, std::string(cols, '.'));
    for (auto &line : grid) {
      for (auto &tile : line) {
        if (pick(10) < 3) {
          tile = '#';
        }
      }
    }
    const auto start = pick(rows * cols);
    auto target = pick(rows * cols - 1);
    if (target >= start) {
      target++;
    }
    grid[start / cols][start % cols] = 'S';
    grid[target / cols][target % cols] = 'E';

    std::string in = std::to_string(rows) + " " + std::to_string(cols) + "\n";
    for (const auto &line : grid) {
      in += line + "\n";
    }
    INFO(in);
    test_succeeds(in, std::to_string(reference_distance(grid)) + "\n");
  }
}

// NOTE: the two tests below are the limits of the problem. A solution that
// recurses once per cell on its way runs out of stack in them, and that stops
// the whole test binary, so they are last

TEST_CASE("handles the largest open grid") {
  constexpr size_t size = 1000;
  std::vector<std::string> grid(size, std::string(size, '.'));
  grid[0][0] = 'S';
  grid[size - 1][size - 1] = 'E';

  std::string in = "1000 1000\n";
  for (const auto &line : grid) {
    in += line + "\n";
  }
  test_succeeds(in, "1998\n");
}

TEST_CASE("handles a path that winds through the whole grid") {
  // NOTE: the even rows are open, and each odd row has one gap at the end the
  // path just came from, so the path is 999 * 500 + 998 moves long
  constexpr size_t rows = 999;
  constexpr size_t cols = 1000;
  std::vector<std::string> grid(rows, std::string(cols, '.'));
  for (size_t row = 1; row < rows; row += 2) {
    grid[row] = std::string(cols, '#');
    grid[row][(row / 2) % 2 == 0 ? cols - 1 : 0] = '.';
  }
  grid[0][0] = 'S';
  grid[rows - 1][0] = 'E';

  std::string in = "999 1000\n";
  for (const auto &line : grid) {
    in += line + "\n";
  }
  test_succeeds(in, "500498\n");
}

namespace {

int reference_distance(const std::vector<std::string> &grid) {
  const auto rows = grid.size();
  const auto cols = grid[0].size();
  std::vector<int> distance(rows * cols, -1);
  std::queue<size_t> queue;
  for (size_t cell = 0; cell < rows * cols; cell++) {
    if (grid[cell / cols][cell % cols] == 'S') {
      distance[cell] = 0;
      queue.push(cell);
    }
  }

  while (!queue.empty()) {
    const auto cell = queue.front();
    queue.pop();
    // NOTE: a step off the top or left wraps around to a huge number, which
    // the bounds check below rejects together with the bottom and right
    const auto visit = [&](size_t row, size_t col) {
      if (row >= rows || col >= cols || grid[row][col] == '#' ||
          distance[row * cols + col] != -1) {
        return;
      }
      distance[row * cols + col] = distance[cell] + 1;
      queue.push(row * cols + col);
    };
    visit(cell / cols - 1, cell % cols);
    visit(cell / cols + 1, cell % cols);
    visit(cell / cols, cell % cols - 1);
    visit(cell / cols, cell % cols + 1);
  }

  for (size_t cell = 0; cell < rows * cols; cell++) {
    if (grid[cell / cols][cell % cols] == 'E') {
      return distance[cell];
    }
  }
  return -1;
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
