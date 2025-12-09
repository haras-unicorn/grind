#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let example = r"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
  ";
  println!("Example:\n{example}\n\n");

  let example_movie_theater = example.parse::<MovieTheater>()?;
  println!("Example movie theater:\n{example_movie_theater}\n");

  let example_max_rectangle_area = example_movie_theater.max_rectangle_area();
  println!("Example max rectangle area:\n{example_max_rectangle_area}\n");

  let example_max_green_rectangle_area =
    example_movie_theater.max_green_rectangle_area();
  println!(
    "Example max green rectangle area:\n{example_max_green_rectangle_area}\n"
  );

  // let input = include_str!("../input.txt");
  // println!("Input:\n{input}\n\n");

  // let input_movie_theater = input.parse::<MovieTheater>()?;
  // println!("Input movie theater:\n{input_movie_theater}\n");

  // let input_max_rectangle_area = input_movie_theater.max_rectangle_area();
  // println!("Input max rectangle area:\n{input_max_rectangle_area}\n");

  // let input_max_green_rectangle_area =
  //   input_movie_theater.max_green_rectangle_area();
  // println!(
  //   "input max green rectangle area:\n{input_max_green_rectangle_area}\n"
  // );

  Ok(())
}

#[derive(Debug, Clone)]
struct MovieTheater {
  red_tiles: Vec<RedTile>,
}

impl MovieTheater {
  fn max_green_rectangle_area(&self) -> Area {
    let mut rectangles = self
      .red_tiles
      .iter()
      .enumerate()
      .flat_map(|(lhs_index, &lhs)| {
        self
          .red_tiles
          .iter()
          .skip(lhs_index.saturating_add(1))
          .map(move |&rhs| Rectangle::new(lhs, rhs))
          .filter(move |&rectangle| {
            let other = rectangle.other_edges();
            [other.lhs, other.rhs].iter().all(|&edge| {
              self.red_tiles.iter().any(|&red_tile| edge == red_tile)
            }) && !self
              .red_tiles
              .iter()
              .any(|&red_tile| rectangle.contains(red_tile))
          })
      })
      .collect::<Vec<_>>();
    rectangles.sort_by(|lhs, rhs| lhs.area.total_cmp(&rhs.area));
    rectangles.reverse();
    rectangles
      .first()
      .map_or(0 as Area, |rectangle| rectangle.area)
  }

  fn max_rectangle_area(&self) -> Area {
    let mut rectangles = self
      .red_tiles
      .iter()
      .enumerate()
      .flat_map(|(lhs_index, &lhs)| {
        self
          .red_tiles
          .iter()
          .skip(lhs_index.saturating_add(1))
          .filter(move |&&rhs| lhs != rhs)
          .map(move |&rhs| Rectangle::new(lhs, rhs))
      })
      .collect::<Vec<_>>();
    rectangles.sort_by(|lhs, rhs| lhs.area.total_cmp(&rhs.area));
    rectangles.reverse();
    rectangles
      .first()
      .map_or(0 as Area, |rectangle| rectangle.area)
  }
}

impl Display for MovieTheater {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.red_tiles.iter().join(RED_TILE_SEPARATOR_STR))
  }
}

impl FromStr for MovieTheater {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let red_tiles = s
      .trim()
      .split(RED_TILE_SEPARATOR_CHAR)
      .map(|red_tile| red_tile.parse::<RedTile>())
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self { red_tiles })
  }
}

const RED_TILE_SEPARATOR_CHAR: char = '\n';
const RED_TILE_SEPARATOR_STR: &str = "\n";

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangle {
  #[allow(dead_code, reason = "useful for debug")]
  lhs: RedTile,
  #[allow(dead_code, reason = "useful for debug")]
  rhs: RedTile,
  area: Area,
}

impl Rectangle {
  fn new(lhs: RedTile, rhs: RedTile) -> Self {
    Self {
      lhs,
      rhs,
      area: (((lhs.x as Area) - (rhs.x as Area)).abs() + 1.0)
        * (((lhs.y as Area) - rhs.y as Area).abs() + 1.0),
    }
  }

  fn other_edges(&self) -> Self {
    Self {
      lhs: RedTile {
        x: self.lhs.x,
        y: self.rhs.y,
      },
      rhs: RedTile {
        x: self.rhs.x,
        y: self.lhs.y,
      },
      area: self.area,
    }
  }

  fn contains(&self, red_tile: RedTile) -> bool {
    let max_x = self.lhs.x.max(self.rhs.x);
    let min_x = self.lhs.x.min(self.rhs.x);
    let min_y = self.lhs.y.min(self.rhs.y);
    let max_y = self.lhs.y.max(self.rhs.y);

    red_tile.x > min_x
      && red_tile.x < max_x
      && red_tile.y > min_y
      && red_tile.y < max_y
  }
}

type Area = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RedTile {
  x: Coordinate,
  y: Coordinate,
}

impl Display for RedTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{COORDINATE_SEPARATOR}{}", self.x, self.y)
  }
}

impl FromStr for RedTile {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let Some((x, y)) = s.trim().split(COORDINATE_SEPARATOR).collect_tuple()
    else {
      return Err(anyhow::anyhow!("invalid red tile"));
    };

    Ok(Self {
      x: x.parse::<Coordinate>()?,
      y: y.parse::<Coordinate>()?,
    })
  }
}

type Coordinate = u64;

const COORDINATE_SEPARATOR: char = ',';
