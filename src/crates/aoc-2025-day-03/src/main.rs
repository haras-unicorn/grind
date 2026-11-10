#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  // let input = r"
  //   987654321111111
  //   811111111111119
  //   234234234234278
  //   818181911112111
  // ";

  let banks = input
    .trim()
    .split('\n')
    .map(|line| line.parse::<Bank>())
    .collect::<Result<Vec<_>, _>>()?;

  println!("Input:\n{}\n", input);
  println!(
    "Banks:\n{}\n",
    banks
      .iter()
      .map(|bank| format!(
        "{} -> {}, {}",
        bank,
        bank.max_joltage(2),
        bank.max_joltage(12)
      ))
      .join("\n")
  );
  println!(
    "Max joltage (2): {}",
    banks.iter().fold(0 as Joltage, |sum, bank| sum
      .saturating_add(bank.max_joltage(2)))
  );
  println!(
    "Max joltage (12): {}",
    banks.iter().fold(0 as Joltage, |sum, bank| sum
      .saturating_add(bank.max_joltage(12)))
  );

  Ok(())
}

#[derive(Debug, Clone)]
struct Bank {
  batteries: Vec<Battery>,
}

impl Bank {
  fn max_joltage(&self, mut num: usize) -> Joltage {
    let mut max_index = Option::<usize>::None;
    let mut sum = 0 as Joltage;
    let mut pow =
      (10 as Joltage).saturating_pow(num.saturating_sub(1) as JoltageExp);

    while num > 0 {
      let Some(next_max_index) =
        (max_index.map(|index| index.saturating_add(1)).unwrap_or(0)
          ..=self.batteries.len().saturating_sub(num))
          .rev()
          .max_by_key(|&index| self.batteries[index])
      else {
        return 0 as Joltage;
      };

      sum = sum.saturating_add(
        self.batteries[next_max_index].joltage.saturating_mul(pow),
      );
      pow = pow.saturating_div(10);
      max_index = Some(next_max_index);
      num = num.saturating_sub(1);
    }

    sum
  }
}

impl Display for Bank {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.batteries.iter().join(""))
  }
}

impl FromStr for Bank {
  type Err = <Battery as TryFrom<char>>::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      batteries: s
        .trim()
        .chars()
        .map(Battery::try_from)
        .collect::<Result<Vec<_>, _>>()?,
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Battery {
  joltage: Joltage,
}

impl Display for Battery {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.joltage)
  }
}

impl FromStr for Battery {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let joltage = s.trim().parse::<Joltage>()?;
    Ok(Self { joltage })
  }
}

impl TryFrom<char> for Battery {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    let joltage = value
      .to_digit(10)
      .ok_or_else(|| anyhow::anyhow!("not a number"))?;
    Ok(Self {
      joltage: joltage as Joltage,
    })
  }
}

type Joltage = u64;
type JoltageExp = u32;
