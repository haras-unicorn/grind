"""Reads characters from stdin and prints the longest unique window length."""

import contextlib
import io
import sys


def main() -> None:
  print(solve(sys.stdin.read()))


def solve(text: str) -> int:
  window: list[str] = []
  best = 0
  for char in text:
    if char in {" ", "\t", "\r", "\n"}:
      continue

    found_index = None
    for index, in_window in enumerate(window):
      if in_window == char:
        found_index = index
        break
    if found_index is not None:
      window = window[found_index + 1 :]
    window.append(char)
    best = max(best, len(window))

  return best


if __name__ == "__main__":
  main()


def test_longest_unique_window() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("abcabcbb") == 3


def test_longest_unique_window_same() -> None:
  assert solve("bbbbb") == 1


def test_longest_unique_window_empty() -> None:
  assert solve("") == 0


def test_sliding_window() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("pwwkew") == 3


def test_repeating_sliding_window() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("cabcaxy") == 5


def test_single_character() -> None:
  assert solve("a") == 1


def test_duplicate_right_after_the_start() -> None:
  assert solve("aab") == 2


def test_duplicate_of_the_first_character() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("dvdf") == 3


def test_window_does_not_move_back() -> None:
  # NOTE: the last 't' repeats a character from before the window started
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("tmmzuxt") == 5


def test_repeat_at_the_far_end() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("abcdefga") == 7


def test_alternating_characters() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("abababab") == 2


def test_longest_window_in_the_middle() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("abcbdea") == 5


def test_whole_alphabet() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert solve("abcdefghijklmnopqrstuvwxyz") == 26


def test_long_input_with_a_repeating_alphabet() -> None:
  alphabet = "abcdefghijklmnopqrstuvwxyz"
  assert solve((alphabet * 7693)[:200000]) == 26


def test_long_input_of_one_character() -> None:
  assert solve("a" * 200000) == 1


def run_main(stdin_text: str) -> str:
  original = sys.stdin
  sys.stdin = io.StringIO(stdin_text)
  try:
    with contextlib.redirect_stdout(io.StringIO()) as output:
      main()
  finally:
    sys.stdin = original
  return output.getvalue()


def test_main_prints_the_answer() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert run_main("pwwkew") == "3\n"


def test_main_ignores_the_final_newline() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert run_main("pwwkew\n") == "3\n"


def test_main_ignores_the_final_newline_of_the_example() -> None:
  assert run_main("bbbbb\n") == "1\n"


def test_main_ignores_windows_line_endings() -> None:
  # cspell:disable-next-line  # noqa: ERA001
  assert run_main("pwwkew\r\n") == "3\n"
