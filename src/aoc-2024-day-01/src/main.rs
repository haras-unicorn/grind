#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let iter = input
    .split("\n")
    .filter(|numbers| !numbers.is_empty())
    .map(|list| {
      #[allow(clippy::unwrap_used, reason = "Static input works")]
      list
        .split("   ")
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let first = iter.iter().map(|item| item[0]).sorted().collect::<Vec<_>>();
  let second = iter.iter().map(|item| item[1]).sorted().collect::<Vec<_>>();

  let distance = first.iter().zip(second.iter()).fold(0u32, |acc, next| {
    acc.saturating_add(next.1.abs_diff(*next.0))
  });

  let similarity = first.iter().fold(0u32, |acc, next| {
    acc.saturating_add(
      (second.iter().filter(|second| **second == *next).count() as u32)
        .saturating_mul(*next),
    )
  });

  println!("Distance {}", distance);
  println!("Similarity {}", similarity);

  Ok(())
}
