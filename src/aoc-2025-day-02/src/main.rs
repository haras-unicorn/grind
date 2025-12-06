#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;
use std::fmt::Display;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  // let input = r"
  //   11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
  //   1698522-1698528,446443-446449,38593856-38593862,565653-565659,
  //   824824821-824824827,2121212118-2121212124
  // ";

  let ranges = input
    .trim()
    .split(',')
    .map(IdRange::parse)
    .collect::<Option<Vec<_>>>()
    .ok_or_else(|| anyhow::anyhow!("invalid id range"))?;

  let invalid_sum = ranges.iter().fold(ID_NUM_0, |sum, range| {
    sum.saturating_add(range.invalid_sum())
  });
  let repeating_invalid_sum = ranges.iter().fold(ID_NUM_0, |sum, range| {
    sum.saturating_add(range.repeating_invalid_sum())
  });

  println!("Input:\n{}\n", input);
  println!("Ranges:\n{}\n", ranges.iter().join("\n"));
  println!("Invalid sum:\n{}\n", invalid_sum);
  println!("Repeating invalid sum:\n{}\n", repeating_invalid_sum);

  Ok(())
}

#[derive(Debug, Copy, Clone)]
struct IdRange<'a, 'b> {
  start: Id<'a>,
  end: Id<'b>,
}

impl<'a> IdRange<'a, 'a> {
  fn parse(text: &'a str) -> Option<IdRange<'a, 'a>> {
    if let Some((start, end)) = text.trim().split('-').collect_tuple() {
      Some(Self {
        start: Id::new(start),
        end: Id::new(end),
      })
    } else {
      None
    }
  }
}

impl<'a, 'b> IdRange<'a, 'b> {
  fn invalid_sum(&self) -> IdNum {
    (self.start.num()..=self.end.num())
      .filter(|&value| Id::new(value.to_string().as_str()).is_invalid())
      .sum()
  }

  fn repeating_invalid_sum(&self) -> IdNum {
    (self.start.num()..=self.end.num())
      .filter(|&value| {
        Id::new(value.to_string().as_str()).is_invalid_repeating()
      })
      .sum()
  }
}

impl<'a, 'b> Display for IdRange<'a, 'b> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.start, self.end)
  }
}

#[derive(Debug, Clone, Copy)]
struct Id<'a>(&'a str);

impl<'a> Id<'a> {
  fn new(value: &'a str) -> Id<'a> {
    Self(value)
  }

  fn text(&self) -> &'a str {
    self.0
  }

  fn num(&self) -> IdNum {
    self.0.parse::<IdNum>().unwrap_or_default()
  }

  fn is_invalid(&self) -> bool {
    let num = self.num();

    let exp = if let Some(exp) =
      (ID_EXP_MIN..ID_EXP_MAX).find(|&exp| ID_NUM_10.saturating_pow(exp) > num)
    {
      exp
    } else {
      return false;
    };

    let half_pow = ID_NUM_10.saturating_pow(exp.saturating_div(2));

    num.checked_div(half_pow) == num.checked_rem(half_pow)
  }

  fn is_invalid_repeating(&self) -> bool {
    let num = self.num();
    let exp = if let Some(exp) =
      (ID_EXP_MIN..ID_EXP_MAX).find(|&exp| ID_NUM_10.saturating_pow(exp) > num)
    {
      exp
    } else {
      return false;
    };

    for parts in 2..=exp {
      let digits = exp.div_ceil(parts);

      let mut last_part = Option::<&'a str>::None;
      let mut parts_all_equal = true;
      for index in 0..parts {
        let start = index.saturating_mul(digits) as usize;
        let end = index.saturating_add(1).saturating_mul(digits) as usize;

        let part = self.text().get(start..end);
        if last_part.is_some() && part != last_part {
          parts_all_equal = false;
          break;
        }
        last_part = part;
      }

      if parts_all_equal {
        return true;
      }
    }

    false
  }
}

impl<'a> Display for Id<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text())
  }
}

type IdNum = u64;
type IdExp = u32;

const ID_NUM_10: IdNum = 10;
const ID_NUM_0: IdNum = 0;

const ID_EXP_MIN: IdExp = 0;
// NOTE: max u64 is 18_446_744_073_709_551_615u64 which has 20 digits
const ID_EXP_MAX: IdExp = 20;
