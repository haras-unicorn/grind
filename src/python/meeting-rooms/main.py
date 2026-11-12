"""Reads meetings from input and returns the smallest number of rooms needed."""

import sys
import time
from collections.abc import Iterator
from heapq import heappush, heappushpop


def main() -> None:
  print(solve(sys.stdin.read()))


def solve(text: str) -> int:
  meetings: list[tuple[int, int]] = []
  for line in text.strip().splitlines()[1:]:
    split = line.strip().split()
    meetings.append((int(split[0]), int(split[1])))
  if len(meetings) == 0:
    return 0
  meetings.sort(key=lambda meeting: meeting[0])

  rooms: list[int] = [meetings[0][1]]
  for meeting in meetings[1:]:
    if meeting[0] < rooms[0]:
      heappush(rooms, meeting[1])
    else:
      heappushpop(rooms, meeting[1])

  return len(rooms)


if __name__ == "__main__":
  main()


def test_example1() -> None:
  assert (
    solve("""
3
0 30
5 10
15 20
""")
    == 2
  )


def test_example2() -> None:
  assert (
    solve("""
2
7 10
2 4
""")
    == 1
  )


def test_example3() -> None:
  assert (
    solve("""
3
1 5
5 8
8 9
""")
    == 1
  )


def test_example4() -> None:
  assert (
    solve("""
4
1 10
2 3
4 5
6 7
""")
    == 2
  )


def make_text(meetings: list[tuple[int, int]]) -> str:
  lines = [str(len(meetings))]
  lines.extend(f"{start} {end}" for start, end in meetings)
  return "\n".join(lines) + "\n"


def reference_rooms(meetings: list[tuple[int, int]]) -> int:
  # NOTE: at the same time, an end sorts before a start, because -1 < 1
  events = sorted(
    [(start, 1) for start, _ in meetings] + [(end, -1) for _, end in meetings]
  )
  rooms = 0
  most = 0
  for _, change in events:
    rooms += change
    most = max(most, rooms)
  return most


def pseudo_random(seed: int) -> Iterator[int]:
  state = seed
  while True:
    state = (state * 1103515245 + 12345) % 2**31
    yield state >> 8


def test_one_meeting() -> None:
  assert solve(make_text([(3, 4)])) == 1


def test_meetings_in_any_order() -> None:
  assert solve(make_text([(5, 6), (1, 2), (3, 4)])) == 1


def test_meetings_that_touch_all_share_one_room() -> None:
  assert solve(make_text([(0, 5), (5, 10), (10, 15), (15, 20)])) == 1


def test_identical_meetings_need_their_own_rooms() -> None:
  assert solve(make_text([(1, 2), (1, 2), (1, 2)])) == 3


def test_nested_meetings() -> None:
  assert solve(make_text([(1, 10), (2, 9), (3, 8), (4, 7)])) == 4


def test_largest_times() -> None:
  assert solve(make_text([(0, 10**9), (10**9 - 1, 10**9)])) == 2


def test_chooses_the_right_free_room() -> None:
  # NOTE: when two rooms are free, which one the meeting takes matters
  meetings = [(1, 2), (1, 5), (6, 10), (3, 11)]
  assert solve(make_text(meetings)) == 2


def test_chooses_the_right_free_room_smallest() -> None:
  meetings = [(9, 13), (11, 12), (7, 10), (5, 8)]
  assert solve(make_text(meetings)) == 2


def test_agrees_with_a_sweep_on_random_meetings() -> None:
  numbers = pseudo_random(1)
  for _ in range(3000):
    count = next(numbers) % 10 + 1
    meetings: list[tuple[int, int]] = []
    for _ in range(count):
      start = next(numbers) % 15
      meetings.append((start, start + 1 + next(numbers) % 10))
    assert solve(make_text(meetings)) == reference_rooms(meetings), meetings


def test_many_staggered_meetings() -> None:
  meetings = [(index, index + 10) for index in range(200000)]
  assert solve(make_text(meetings)) == 10


def test_many_overlapping_meetings_do_not_take_long() -> None:
  # NOTE: a solution that looks through every room for every meeting takes
  # minutes at the largest input, and about this long at this size
  meetings = [(0, 10**9)] * 30000
  started = time.perf_counter()
  assert solve(make_text(meetings)) == 30000
  assert time.perf_counter() - started < 1.0
