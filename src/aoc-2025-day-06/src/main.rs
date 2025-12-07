#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  //   let input = r"
  // 123 328  51 64
  //  45 64  387 23
  //   6 98  215 314
  // *   +   *   +  "
  //     .split('\n')
  //     .skip(1)
  //     .join("\n");

  println!("Input:\n'{input}'");

  let worksheet = input.parse::<Worksheet>()?;
  let solution = worksheet.solve();

  println!("Worksheet:\n{worksheet}");
  println!("Solution: {solution}");

  let corrected_worksheet = input.parse::<CorrectedWorksheet>()?;
  let corrected_solution = corrected_worksheet.solve();

  println!("\nCorrected worksheet:\n{corrected_worksheet}");
  println!("Corrected solution: {corrected_solution}");

  Ok(())
}

#[derive(Debug, Clone)]
struct CorrectedWorksheet {
  problems: Vec<Problem>,
}

impl CorrectedWorksheet {
  fn solve(&self) -> Operand {
    self.problems.iter().fold(0 as Operand, |sum, problem| {
      sum.saturating_add(problem.solve())
    })
  }
}

impl Display for CorrectedWorksheet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_width = self
      .problems
      .iter()
      .flat_map(|p| &p.operands)
      .map(|n| n.to_string().len())
      .max()
      .unwrap_or(1);
    #[allow(clippy::unwrap_used, reason = "worksheet cannot be empty")]
    let max_operands = self
      .problems
      .iter()
      .map(|problem| problem.operands.len())
      .max()
      .unwrap();

    write!(
      f,
      "{}\n{}",
      (0..max_operands)
        .map(|j| (0..self.problems.len())
          .map(|i| format!(
            "{:>width$}",
            self.problems[i].operands.get(j).unwrap_or(&0),
            width = max_width
          ))
          .join(" "))
        .join("\n"),
      (0..self.problems.len())
        .map(|i| format!(
          "{:>width$}",
          self.problems[i].operation,
          width = max_width
        ))
        .join(" ")
    )
  }
}

impl FromStr for CorrectedWorksheet {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let operand_lengths = s
      .split('\n')
      .filter(|line| !line.is_empty())
      .next_back()
      .ok_or_else(|| anyhow::anyhow!("worksheet cannot be empty"))?
      .chars()
      .fold(
        (vec![], 0_usize, false),
        |(mut operand_lengths, mut length, mut space), char| {
          if space && !char.is_whitespace() {
            operand_lengths.push(length.saturating_sub(1));
            length = 0;
            space = false;
          } else if char.is_whitespace() {
            space = true;
          }
          length = length.saturating_add(1);
          (operand_lengths, length, space)
        },
      );
    #[allow(clippy::unwrap_used, reason = "worksheet cannot be empty")]
    let last_operand_length = s
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|line| {
        let mut current = line.len().saturating_sub(1);
        let mut space = true;
        loop {
          #[allow(clippy::unwrap_used, reason = "worksheet cannot be empty")]
          let char = line.chars().nth(current).unwrap();
          if !char.is_whitespace() {
            space = false;
          } else if char.is_whitespace() && !space {
            break;
          }
          current = current.saturating_sub(1);
        }
        line.len().saturating_sub(current).saturating_sub(1)
      })
      .max()
      .unwrap();
    let mut operand_lengths = operand_lengths.0;
    operand_lengths.push(last_operand_length);

    let table = s
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|line| {
        operand_lengths
          .iter()
          .fold((Vec::new(), 0_usize), |(mut acc, start), &next| {
            if let Some(chunk) = line.get(start..(start.saturating_add(next))) {
              acc.push(chunk);
              (acc, (start.saturating_add(next).saturating_add(1)))
            } else {
              #[allow(
                clippy::unwrap_used,
                reason = "building off of existing lengths"
              )]
              let chunk = line.get(start..(start.saturating_add(1))).unwrap();
              acc.push(chunk);
              (acc, (start.saturating_add(1)))
            }
          })
          .0
      })
      .collect::<Vec<_>>();
    if table.is_empty() {
      return Err(anyhow::anyhow!("worksheet cannot be empty"));
    }
    if table.len() < 2 {
      return Err(anyhow::anyhow!("missing operations or operands in\n{s}"));
    }

    let problems = (0..table[0].len())
      .map(|problem_index| -> Result<Problem, Self::Err> {
        Ok(Problem {
          operands: (0..operand_lengths[problem_index])
            .map(|operand_index| {
              (0..table.len().saturating_sub(1)).fold(
                String::new(),
                |operand, row_index| {
                  table[row_index][problem_index]
                    .chars()
                    .nth(operand_index)
                    .map_or_else(
                      || operand.to_owned(),
                      |char| {
                        let mut operand = String::from(&operand);
                        if char.is_ascii_digit() {
                          operand.push(char);
                        }
                        operand
                      },
                    )
                },
              )
            })
            .map(|operand_string| operand_string.parse::<Operand>())
            .collect::<Result<Vec<_>, _>>()?,
          operation: table[table.len().saturating_sub(1)][problem_index]
            .parse::<Operation>()?,
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self { problems })
  }
}

#[derive(Debug, Clone)]
struct Worksheet {
  problems: Vec<Problem>,
}

impl Worksheet {
  fn solve(&self) -> Operand {
    self.problems.iter().fold(0 as Operand, |sum, problem| {
      sum.saturating_add(problem.solve())
    })
  }
}

impl Display for Worksheet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_width = self
      .problems
      .iter()
      .flat_map(|p| &p.operands)
      .map(|n| n.to_string().len())
      .max()
      .unwrap_or(1);

    write!(
      f,
      "{}\n{}",
      (0..self.problems[0].operands.len())
        .map(|j| (0..self.problems.len())
          .map(|i| format!(
            "{:>width$}",
            self.problems[i].operands[j],
            width = max_width
          ))
          .join(" "))
        .join("\n"),
      (0..self.problems.len())
        .map(|i| format!(
          "{:>width$}",
          self.problems[i].operation,
          width = max_width
        ))
        .join(" ")
    )
  }
}

impl FromStr for Worksheet {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let table = s
      .trim()
      .split('\n')
      .map(|line| {
        line
          .trim()
          .split(' ')
          .filter(|item| !item.is_empty())
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    if table.is_empty() {
      return Err(anyhow::anyhow!("worksheet cannot be empty"));
    }
    if table.len() < 2 {
      return Err(anyhow::anyhow!("missing operations or operands in\n{s}"));
    }

    Ok(Self {
      problems: (0..table[0].len())
        .map(|j| -> Result<Problem, Self::Err> {
          Ok(Problem {
            operands: (0..table.len().saturating_sub(1))
              .map(|i| table[i][j].parse::<Operand>())
              .collect::<Result<Vec<_>, _>>()?,
            operation: table[table.len().saturating_sub(1)][j]
              .parse::<Operation>()?,
          })
        })
        .collect::<Result<Vec<_>, _>>()?,
    })
  }
}

#[derive(Debug, Clone)]
struct Problem {
  operands: Vec<Operand>,
  operation: Operation,
}

impl Problem {
  fn solve(&self) -> Operand {
    self
      .operands
      .iter()
      .fold(self.operation.initial(), |acc, &next| {
        self.operation.apply(acc, next)
      })
  }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
  Sum,
  Product,
}

impl Operation {
  fn initial(self) -> Operand {
    match self {
      Operation::Sum => 0,
      Operation::Product => 1,
    }
  }

  fn apply(self, lhs: Operand, rhs: Operand) -> Operand {
    match self {
      Operation::Sum => lhs.saturating_add(rhs),
      Operation::Product => lhs.saturating_mul(rhs),
    }
  }
}

impl Display for Operation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      Operation::Sum => SUM_OPERATION_STR,
      Operation::Product => PRODUCT_OPERATION_STR,
    };

    f.pad(symbol)
  }
}

impl FromStr for Operation {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.chars().nth(0) {
      Some(SUM_OPERATION_CHAR) => Ok(Operation::Sum),
      Some(PRODUCT_OPERATION_CHAR) => Ok(Operation::Product),
      _ => Err(anyhow::anyhow!("invalid operation {s}")),
    }
  }
}

impl TryFrom<char> for Operation {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      SUM_OPERATION_CHAR => Ok(Operation::Sum),
      PRODUCT_OPERATION_CHAR => Ok(Operation::Product),
      _ => Err(anyhow::anyhow!("invalid operation {value}")),
    }
  }
}

const SUM_OPERATION_CHAR: char = '+';
const PRODUCT_OPERATION_CHAR: char = '*';

const SUM_OPERATION_STR: &str = "+";
const PRODUCT_OPERATION_STR: &str = "*";

type Operand = u64;
