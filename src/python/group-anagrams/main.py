"""Reads the input from stdin and prints the answer to stdout."""

import sys
import time
from collections.abc import Iterator


def main() -> None:
  print(solve(sys.stdin.read()))


def solve(text: str) -> str:
  words = text.strip().split()[1:]
  anagrams = ["".join(sorted(word)) for word in words]
  groups: dict[str, list[str]] = {}

  for word, anagram in zip(words, anagrams, strict=True):
    group = groups.get(anagram)
    if group is None:
      groups[anagram] = [word]
    else:
      group.append(word)

  return "\n".join(" ".join(group) for group in groups.values())


if __name__ == "__main__":
  main()


def test_example1() -> None:
  assert (
    solve("""
6
eat tea tan ate nat bat
""")
    == """
eat tea ate
tan nat
bat
""".strip()
  )


def test_example2() -> None:
  assert (
    solve("""
1
a
""")
    == """
a
""".strip()
  )


def test_example3() -> None:
  assert (
    solve("""
4
ab ba ab c
""")
    == """
ab ba ab
c
""".strip()
  )


def make_text(words: list[str]) -> str:
  return f"{len(words)}\n{' '.join(words)}\n"


def reference_groups(words: list[str]) -> str:
  groups: dict[str, list[str]] = {}
  for word in words:
    groups.setdefault("".join(sorted(word)), []).append(word)
  return "\n".join(" ".join(group) for group in groups.values())


def pseudo_random(seed: int) -> Iterator[int]:
  state = seed
  while True:
    state = (state * 1103515245 + 12345) % 2**31
    yield state >> 8


def random_word(numbers: Iterator[int], length: int, letters: int) -> str:
  return "".join(chr(ord("a") + next(numbers) % letters) for _ in range(length))


def test_words_on_several_lines() -> None:
  assert solve("5\neat tea\ntan\nate nat\n") == "eat tea ate\ntan nat"


def test_extra_spaces_and_blank_lines() -> None:
  assert solve("3\n\n  ab   ba \n\n c \n") == "ab ba\nc"


def test_groups_are_ordered_by_their_first_word() -> None:
  assert solve(make_text(["b", "a", "ab", "ba", "c"])) == "b\na\nab ba\nc"


def test_the_same_word_repeated() -> None:
  assert solve(make_text(["abc", "abc", "abc"])) == "abc abc abc"


def test_no_anagrams() -> None:
  assert solve(make_text(["a", "b", "c"])) == "a\nb\nc"


def test_the_number_of_each_letter_matters() -> None:
  assert solve(make_text(["aab", "abb", "aba"])) == "aab aba\nabb"


def test_longest_words() -> None:
  first = "a" * 50 + "b" * 50
  second = "b" * 50 + "a" * 50
  third = "a" * 51 + "b" * 49
  assert (
    solve(make_text([first, second, third])) == f"{first} {second}\n{third}"
  )


def test_agrees_with_a_reference_on_random_words() -> None:
  numbers = pseudo_random(1)
  for _ in range(2000):
    words = [
      random_word(numbers, next(numbers) % 4 + 1, 3)
      for _ in range(next(numbers) % 30 + 1)
    ]
    assert solve(make_text(words)) == reference_groups(words), words


def test_many_words_in_one_group() -> None:
  base = "abcdefgh"
  words = [base[index % 8 :] + base[: index % 8] for index in range(100000)]
  assert solve(make_text(words)) == " ".join(words)


def test_many_groups_do_not_take_long() -> None:
  # NOTE: a solution that compares every word with every group takes minutes
  # at the largest input, and several seconds at this size
  numbers = pseudo_random(2)
  words = [random_word(numbers, 8, 26) for _ in range(20000)]
  started = time.perf_counter()
  assert solve(make_text(words)) == reference_groups(words)
  assert time.perf_counter() - started < 1.0
