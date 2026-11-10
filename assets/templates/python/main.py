"""Reads the input from stdin and prints the answer to stdout."""

import sys


def main() -> None:
  print(solve(sys.stdin.read()))


def solve(text: str) -> int:
  return sum(int(word) for word in text.split())


if __name__ == "__main__":
  main()


def test_sums_the_numbers() -> None:
  assert solve("1 2\n3\n") == 6


def test_sums_nothing_to_zero() -> None:
  assert solve("") == 0
