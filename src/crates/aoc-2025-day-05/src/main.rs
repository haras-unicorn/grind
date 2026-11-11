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
  //   3-5
  //   10-14
  //   16-20
  //   12-18

  //   1
  //   5
  //   8
  //   11
  //   17
  //   32
  // ";

  let database = input.parse::<Database>()?;
  let fresh_ingredients = database.count_fresh_ingredients();
  let possible_fresh_ingredients = database.count_possible_fresh_ingredients();

  println!("Input:\n{input}\n");
  println!("Database:\n{database}\n");
  println!("Fresh ingredients: {fresh_ingredients}");
  println!("Possible fresh ingredients: {possible_fresh_ingredients}");

  Ok(())
}

#[derive(Debug, Clone)]
struct Database {
  ranges: Vec<IngredientRange>,
  ingredients: Vec<IngredientId>,
}

impl Database {
  fn count_possible_fresh_ingredients(&self) -> usize {
    let mut sorted_ranges = self.ranges.clone();
    sorted_ranges.sort_by_key(|range| range.start);
    let sorted_ranges = sorted_ranges;
    if sorted_ranges.is_empty() {
      return 0;
    }

    let mut window = IngredientRange {
      start: sorted_ranges[0].start,
      end: sorted_ranges[0].end,
    };
    let mut count = window.end.saturating_sub(window.start).saturating_add(1);
    for range in sorted_ranges.iter().skip(1) {
      if window.end < range.start {
        count = count
          .saturating_add(range.end.saturating_sub(range.start))
          .saturating_add(1);
        window = *range;
      } else if range.end > window.end {
        count = count.saturating_add(range.end.saturating_sub(window.end));
        window.end = range.end;
      }
    }

    count as usize
  }

  fn count_fresh_ingredients(&self) -> usize {
    let mut count = 0_usize;

    for &ingredient in self.ingredients.iter() {
      for range in self.ranges.iter() {
        if ingredient >= range.start && ingredient <= range.end {
          count = count.saturating_add(1);
          break;
        }
      }
    }

    count
  }
}

impl Display for Database {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\n\n{}",
      self.ranges.iter().join("\n"),
      self.ingredients.iter().join("\n"),
    )
  }
}

impl FromStr for Database {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let Some((ranges, ingredients)) = s.trim().split("\n\n").collect_tuple()
    else {
      return Err(anyhow::anyhow!("Invalid database {s}"));
    };

    let ranges = ranges
      .split("\n")
      .map(IngredientRange::from_str)
      .collect::<Result<Vec<_>, _>>()?;
    let ingredients = ingredients
      .split("\n")
      .map(|ingredient| ingredient.trim().parse::<IngredientId>())
      .collect::<Result<Vec<_>, _>>()
      .map_err(|err| anyhow::anyhow!("Invalid ingredient id: {err}"))?;

    Ok(Self {
      ranges,
      ingredients,
    })
  }
}

#[derive(Debug, Clone, Copy)]
struct IngredientRange {
  start: IngredientId,
  end: IngredientId,
}

impl Display for IngredientRange {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.start, self.end)
  }
}

impl FromStr for IngredientRange {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let Some((Ok(start), Ok(end))) = s
      .trim()
      .split('-')
      .map(|id| id.parse::<IngredientId>())
      .collect_tuple()
    else {
      return Err(anyhow::anyhow!("Invalid range {s}"));
    };

    Ok(Self { start, end })
  }
}

type IngredientId = u64;
