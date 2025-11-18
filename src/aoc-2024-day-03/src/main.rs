#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let multiply_statements = MULTIPLY_STATEMENT_REGEX
    .captures_iter(input)
    .map(|r#match| {
      #[allow(clippy::unwrap_used, reason = "Static regex works")]
      let parsed = (
        r#match.get(0).unwrap().start(),
        r#match.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        r#match.get(2).unwrap().as_str().parse::<u32>().unwrap(),
      );

      parsed
    })
    .collect::<Vec<_>>();

  let do_statements = DO_STATEMENT_REGEX
    .captures_iter(input)
    .map(|r#match| {
      #[allow(clippy::unwrap_used, reason = "Static regex works")]
      let start = r#match.get(0).unwrap().start();
      start
    })
    .collect::<Vec<_>>();

  let do_not_statements = DO_NOT_STATEMENT_REGEX
    .captures_iter(input)
    .map(|r#match| {
      #[allow(clippy::unwrap_used, reason = "Static regex works")]
      let start = r#match.get(0).unwrap().start();
      start
    })
    .collect::<Vec<_>>();

  let sum = multiply_statements.iter().fold(0u32, |acc, next| {
    let x = next.1;
    let y = next.2;
    acc.saturating_add(x.saturating_mul(y))
  });

  let conditional = multiply_statements.iter().fold(0u32, |acc, next| {
    let start = next.0;

    let previous_do = do_statements.iter().rev().find(|r#do| **r#do < start);
    let previous_do_not = do_not_statements
      .iter()
      .rev()
      .find(|do_not| **do_not < start);

    let x = next.1;
    let y = next.2;
    let mul = x.saturating_mul(y);

    match (previous_do, previous_do_not) {
      (None, None) => acc.saturating_add(mul),
      (None, Some(_)) => acc,
      (Some(_), None) => acc.saturating_add(mul),
      (Some(previous_do), Some(previous_do_not)) => {
        if previous_do > previous_do_not {
          acc.saturating_add(mul)
        } else {
          acc
        }
      }
    }
  });

  println!("Sum: {}", sum);
  println!("Conditional: {}", conditional);

  Ok(())
}

lazy_static::lazy_static! {
  static ref MULTIPLY_STATEMENT_REGEX: regex::Regex = {
    #[allow(clippy::unwrap_used, reason = "Valid regex")]
    let regex = regex::Regex::new(r"mul\(([1-9][0-9]*),([1-9][0-9]*)\)").unwrap();
    regex
  };

  static ref DO_STATEMENT_REGEX: regex::Regex = {
    #[allow(clippy::unwrap_used, reason = "Valid regex")]
    let regex = regex::Regex::new(r"do\(\)").unwrap();
    regex
  };

  static ref DO_NOT_STATEMENT_REGEX: regex::Regex = {
    #[allow(clippy::unwrap_used, reason = "Valid regex")]
    let regex = regex::Regex::new(r"don't\(\)").unwrap();
    regex
  };
}
