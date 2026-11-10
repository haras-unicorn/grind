#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let map = input
    .trim()
    .split("\n")
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|char| {
          #[allow(clippy::unwrap_used, reason = "Static input")]
          let height = char.to_digit(10).unwrap() as i32;
          height
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let height = map.len();
  let width = map[0].len();

  let input_trailheads = (0..height)
    .cartesian_product(0..width)
    .filter(|(y, x)| map[*y][*x] == 0)
    .map(|(y, x)| ((y, x), vec![(y, x)]))
    .collect::<Vec<_>>();

  let mut score_trailheads = input_trailheads.clone();
  walk(&map, &mut score_trailheads, true);

  let mut rating_trailheads = input_trailheads.clone();
  walk(&map, &mut rating_trailheads, false);

  let score = score_trailheads
    .iter()
    .flat_map(|(_, positions)| positions.iter())
    .count();

  let rating = rating_trailheads
    .iter()
    .flat_map(|(_, positions)| positions.iter())
    .count();

  println!("Score: {}", score);

  println!("Rating: {}", rating);

  Ok(())
}

type Position = (usize, usize);
type Trailhead = Vec<(Position, Vec<Position>)>;
type Map = Vec<Vec<i32>>;

fn walk(map: &Map, trailheads: &mut Trailhead, unique: bool) {
  for (_, ref mut positions) in trailheads.iter_mut() {
    for _ in 0..9usize {
      let mut new_positions = Vec::new();
      for (y, x) in positions.iter().cloned() {
        for [dy, dx] in [[0, 1], [1, 0], [0, -1], [-1, 0]] {
          let ty = TryInto::<i32>::try_into(y)
            .map(|y| y.saturating_add(dy))
            .and_then(TryInto::<usize>::try_into);
          let tx = TryInto::<i32>::try_into(x)
            .map(|x| x.saturating_add(dx))
            .and_then(TryInto::<usize>::try_into);
          if let (Ok(ty), Ok(tx)) = (ty, tx) {
            if map.get(ty).and_then(|line| line.get(tx).cloned())
              == Some(map[y][x].saturating_add(1))
            {
              new_positions.push((ty, tx))
            }
          }
        }
      }
      if unique {
        *positions = new_positions.into_iter().unique().collect::<Vec<_>>();
      } else {
        *positions = new_positions;
      }
    }
  }
}
