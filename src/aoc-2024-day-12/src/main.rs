#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let map = input
    .trim()
    .split("\n")
    .map(|line| line.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let height = map.len();
  let width = map[0].len();

  let mut regions = Vec::<Region>::new();

  for position in (0..height)
    .cartesian_product(0..width)
    .map(|(y, x)| Position { y, x })
  {
    if regions
      .iter()
      .any(|region| region.positions.contains(&position))
    {
      continue;
    }

    regions.push(Region::new(&map, position));
  }

  let price_perimeter = regions
    .iter()
    .map(|region| region.price_perimeter())
    .sum::<Price>();

  let price_sides = regions
    .iter()
    .map(|region| region.price_sides())
    .sum::<Price>();

  println!("Price perimeter: {}", price_perimeter);

  println!("Price sides: {}", price_sides);

  Ok(())
}

type Plant = char;

type Area = u64;
type Perimeter = u64;
type Side = u64;
type Price = u64;

type Map = Vec<Vec<Plant>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
  y: usize,
  x: usize,
}

impl Position {
  fn diff(self, dy: i32, dx: i32) -> Option<Position> {
    let y = TryInto::<usize>::try_into((self.y as i32).saturating_add(dy));
    let x = TryInto::<usize>::try_into((self.x as i32).saturating_add(dx));
    if let (Ok(y), Ok(x)) = (y, x) {
      return Some(Position { y, x });
    }

    None
  }
}

#[derive(Debug, Clone)]
struct Region {
  plant: Plant,
  positions: HashSet<Position>,
  area: Area,
  perimeter: Perimeter,
  sides: Side,
}

impl Region {
  fn new(map: &Map, start: Position) -> Self {
    let plant = map[start.y][start.x];
    let mut positions = HashSet::new();
    positions.insert(start);
    let mut region = Region {
      plant,
      positions,
      area: 1,
      perimeter: 0,
      sides: 0,
    };
    region.fill(map, start);
    region
  }

  fn price_perimeter(&self) -> Price {
    self.area.saturating_mul(self.perimeter)
  }

  fn price_sides(&self) -> Price {
    self.area.saturating_mul(self.sides)
  }

  fn fill(&mut self, map: &Map, current: Position) {
    for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
      let next = current.diff(dy, dx);
      if let Some(next) = next {
        if self.positions.contains(&next) {
          continue;
        }
        if let Some(plant) = map_get(map, next) {
          if plant == self.plant {
            self.area = self.area.saturating_add(1);
            self.positions.insert(next);
            self.fill(map, next);
            continue;
          }
        }
      }
      self.perimeter = self.perimeter.saturating_add(1);
      let (sdy, sdx) = if dy == 0 { (1, 0) } else { (0, 1) };
      let side_current = current
        .diff(sdy, sdx)
        .and_then(|side_current| map_get(map, side_current));
      let side_next = next
        .and_then(|next| next.diff(sdy, sdx))
        .and_then(|side_next| map_get(map, side_next));
      if ((side_current, side_next) == (Some(self.plant), Some(self.plant)))
        || (side_current != Some(self.plant))
      {
        self.sides = self.sides.saturating_add(1);
      }
    }
  }
}

fn map_get(map: &Map, p: Position) -> Option<Plant> {
  map.get(p.y).and_then(|line| line.get(p.x)).copied()
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.y, self.x)
  }
}

impl Display for Region {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Region {{ plant: {:?}, area: {:?}, perimeter: {:?}, sides: {:?} }}",
      self.plant, self.area, self.perimeter, self.sides
    )
  }
}
