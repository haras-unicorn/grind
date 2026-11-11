//! Reads the input from stdin and prints the answer to stdout.

use std::io::Read;

#[allow(
  clippy::print_stdout,
  reason = "the answer of the problem goes to stdout"
)]
fn main() -> anyhow::Result<()> {
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input)?;

  let answer = solve(&input)?;
  println!("{answer}");

  Ok(())
}

fn solve(input: &str) -> Result<i64, Error> {
  input
    .split_whitespace()
    .map(parse_number)
    .try_fold(0_i64, |sum, number| {
      sum.checked_add(number?).ok_or(Error::Overflow)
    })
}

fn parse_number(word: &str) -> Result<i64, Error> {
  word.parse().map_err(|source| Error::NotANumber {
    word: word.to_owned(),
    source,
  })
}

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error("{word:?} is not a number")]
  NotANumber {
    word: String,
    source: std::num::ParseIntError,
  },
  #[error("the sum does not fit in 64 bits")]
  Overflow,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sums_the_numbers() {
    assert_eq!(solve("1 2\n3\n").unwrap(), 6);
  }

  #[test]
  fn rejects_a_word_that_is_not_a_number() {
    assert!(matches!(solve("one"), Err(Error::NotANumber { .. })));
  }

  #[test]
  fn reports_an_overflow() {
    let input = format!("{} 1", i64::MAX);
    assert!(matches!(solve(&input), Err(Error::Overflow)));
  }
}
