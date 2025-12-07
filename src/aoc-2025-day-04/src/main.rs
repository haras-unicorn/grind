#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  // let input = r"
  //   ..@@.@@@@.
  //   @@@.@.@.@@
  //   @@@@@.@.@@
  //   @.@@@@..@.
  //   @@.@@@@.@@
  //   .@@@@@@@.@
  //   .@.@.@.@@@
  //   @.@@@.@@@@
  //   .@@@@@@@@.
  //   @.@.@@@.@.
  // ";

  let printing_department = input.parse::<PrintingDepartment>()?;

  println!("Input:\n{}\n", input);
  println!("Printing department:\n{}\n", printing_department);
  println!(
    "Accessible by forklift: {}",
    printing_department.accessible_by_forklift_count()
  );
  println!(
    "Accessible by forklift repeating: {}",
    printing_department.accessible_by_forklift_repeating_count()
  );

  Ok(())
}

#[derive(Debug, Clone, Copy)]
struct PrintingDepartmentItem {
  #[allow(dead_code, reason = "nice to have")]
  tile: PrintingDepartmentTile,
  x: usize,
  y: usize,
}

#[derive(Debug, Clone, Copy)]
struct PrintingDepartmentIter<'a> {
  printing_department: &'a PrintingDepartment,
  x: usize,
  max_x: usize,
  y: usize,
  max_y: usize,
}

impl<'a> Iterator for PrintingDepartmentIter<'a> {
  type Item = PrintingDepartmentItem;

  fn next(&mut self) -> Option<Self::Item> {
    if self.y > self.max_y {
      return None;
    }

    let result =
      Some(self.printing_department.get(self.x, self.y)).map(|tile| {
        PrintingDepartmentItem {
          tile,
          x: self.x,
          y: self.y,
        }
      });

    if self.x == self.max_x {
      self.x = 0;
      self.y = self.y.saturating_add(1);
    } else {
      self.x = self.x.saturating_add(1);
    }

    result
  }
}

#[derive(Debug, Clone)]
struct PrintingDepartment {
  tiles: Vec<Vec<PrintingDepartmentTile>>,
}

impl<'a> IntoIterator for &'a PrintingDepartment {
  type Item = PrintingDepartmentItem;

  type IntoIter = PrintingDepartmentIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      printing_department: self,
      x: 0,
      max_x: self.tiles[0].len().saturating_sub(2),
      y: 0,
      max_y: self.tiles.len().saturating_sub(2),
    }
  }
}

impl PrintingDepartment {
  fn accessible_by_forklift_repeating_count(&self) -> usize {
    let mut current = self.clone();
    let mut next = current.clone();
    let mut result = 0_usize;
    loop {
      let mut changed = false;
      for item in current.into_iter() {
        if item.tile == PrintingDepartmentTile::Paper
          && current.adjacent_paper_count(item.x, item.y) < 4
        {
          next.set(item.x, item.y, PrintingDepartmentTile::Empty);
          result = result.saturating_add(1);
          changed = true;
        }
      }
      if !changed {
        break;
      }
      current = next;
      next = current.clone();
    }
    result
  }

  fn accessible_by_forklift_count(&self) -> usize {
    self
      .into_iter()
      .filter(|item| {
        item.tile == PrintingDepartmentTile::Paper
          && self.adjacent_paper_count(item.x, item.y) < 4
      })
      .count()
  }

  fn adjacent_paper_count(&self, x: usize, y: usize) -> usize {
    let x = x.saturating_add(1);
    let y = y.saturating_add(1);

    [
      self.get_unfixed(x, y.saturating_sub(1)),
      self.get_unfixed(x.saturating_add(1), y.saturating_sub(1)),
      self.get_unfixed(x.saturating_add(1), y),
      self.get_unfixed(x.saturating_add(1), y.saturating_add(1)),
      self.get_unfixed(x, y.saturating_add(1)),
      self.get_unfixed(x.saturating_sub(1), y.saturating_add(1)),
      self.get_unfixed(x.saturating_sub(1), y),
      self.get_unfixed(x.saturating_sub(1), y.saturating_sub(1)),
    ]
    .iter()
    .filter(|&&tile| tile == PrintingDepartmentTile::Paper)
    .count()
  }

  fn get(&self, x: usize, y: usize) -> PrintingDepartmentTile {
    self.get_unfixed(x.saturating_add(1), y.saturating_add(1))
  }

  fn get_unfixed(&self, x: usize, y: usize) -> PrintingDepartmentTile {
    self
      .tiles
      .get(y)
      .and_then(|line| line.get(x).cloned())
      .unwrap_or(PrintingDepartmentTile::Empty)
  }

  fn set(&mut self, x: usize, y: usize, tile: PrintingDepartmentTile) {
    self.set_unfixed(x.saturating_add(1), y.saturating_add(1), tile)
  }

  fn set_unfixed(&mut self, x: usize, y: usize, tile: PrintingDepartmentTile) {
    if x > self.tiles.len().saturating_sub(1) {
      return;
    }

    if y > self.tiles[x].len().saturating_sub(1) {
      return;
    }

    self.tiles[y][x] = tile
  }
}

impl Display for PrintingDepartment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self
        .tiles
        .iter()
        .skip(1)
        .take(self.tiles.len().saturating_sub(2))
        .map(|line| line
          .iter()
          .skip(1)
          .take(line.len().saturating_sub(2))
          .join(""))
        .join("\n")
    )
  }
}

impl FromStr for PrintingDepartment {
  type Err = <PrintingDepartmentTile as TryFrom<char>>::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut tiles = s
      .trim()
      .split("\n")
      .map(|line| -> Result<Vec<PrintingDepartmentTile>, Self::Err> {
        let mut line = line
          .trim()
          .chars()
          .map(PrintingDepartmentTile::try_from)
          .collect::<Result<Vec<_>, _>>()?;

        line.insert(0, PrintingDepartmentTile::Empty);
        line.push(PrintingDepartmentTile::Empty);

        Ok(line)
      })
      .collect::<Result<Vec<_>, _>>()?;

    if tiles.is_empty() {
      return Ok(Self {
        tiles: vec![
          vec![PrintingDepartmentTile::Empty, PrintingDepartmentTile::Empty],
          vec![PrintingDepartmentTile::Empty, PrintingDepartmentTile::Empty],
        ],
      });
    }

    tiles.insert(
      0,
      (0..tiles[0].len())
        .map(|_| PrintingDepartmentTile::Empty)
        .collect::<Vec<_>>(),
    );

    tiles.push(
      (0..tiles[0].len())
        .map(|_| PrintingDepartmentTile::Empty)
        .collect::<Vec<_>>(),
    );

    Ok(Self { tiles })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PrintingDepartmentTile {
  Empty,
  Paper,
}

impl Display for PrintingDepartmentTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PrintingDepartmentTile::Empty => {
        write!(f, "{}", EMPTY_PRINTING_DEPARTMENT_TILE_STR)
      }
      PrintingDepartmentTile::Paper => {
        write!(f, "{}", PAPER_PRINTING_DEPARTMENT_TILE_STR)
      }
    }
  }
}

impl TryFrom<char> for PrintingDepartmentTile {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      EMPTY_PRINTING_DEPARTMENT_TILE_CHAR => Ok(PrintingDepartmentTile::Empty),
      PAPER_PRINTING_DEPARTMENT_TILE_CHAR => Ok(PrintingDepartmentTile::Paper),
      _ => Err(anyhow::anyhow!("not a valid tile")),
    }
  }
}

impl FromStr for PrintingDepartmentTile {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.get(0..1) {
      Some(EMPTY_PRINTING_DEPARTMENT_TILE_STR) => {
        Ok(PrintingDepartmentTile::Empty)
      }
      Some(PAPER_PRINTING_DEPARTMENT_TILE_STR) => {
        Ok(PrintingDepartmentTile::Paper)
      }
      _ => Err(anyhow::anyhow!("not a valid tile")),
    }
  }
}

const EMPTY_PRINTING_DEPARTMENT_TILE_STR: &str = ".";
const PAPER_PRINTING_DEPARTMENT_TILE_STR: &str = "@";

const EMPTY_PRINTING_DEPARTMENT_TILE_CHAR: char = '.';
const PAPER_PRINTING_DEPARTMENT_TILE_CHAR: char = '@';
