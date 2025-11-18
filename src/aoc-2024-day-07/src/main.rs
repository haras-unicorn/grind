#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::convert::identity;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt").trim();

  // let input = r"
  //   190: 10 19
  //   3267: 81 40 27
  //   83: 17 5
  //   156: 15 6
  //   7290: 6 8 6 15
  //   161011: 16 10 13
  //   192: 17 8 14
  //   21037: 9 7 18 13
  //   292: 11 6 16 20
  // "
  // .trim();

  let equations = input
    .split("\n")
    .map(|equation| -> anyhow::Result<Equation> {
      let (solution, operands) = equation
        .trim()
        .split_once(":")
        .ok_or_else(|| anyhow::anyhow!("Failed parsing equation"))?;

      let solution = solution.trim().parse::<u64>()?;

      let operands = operands
        .trim()
        .split(" ")
        .map(|operand| operand.trim().parse::<u64>())
        .process_results(|operands| operands.collect::<Vec<_>>())?;

      let num_operators = operands.len().saturating_sub(1);

      let sum_mul_operators = (0..(2u64.saturating_pow(num_operators as u32)))
        .map(|combination| {
          let base_2 = pad::PadStr::pad(
            format!("{}", radix_fmt::radix(combination, 2)).as_str(),
            num_operators,
            '0',
            pad::Alignment::Right,
            false,
          );
          (0..(num_operators))
            .map(|operator| {
              let digit = base_2.chars().nth(operator).unwrap_or('0');
              if digit == '0' {
                '+'
              } else {
                '*'
              }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

      let sum_mul_concat_operators = (0..(3u64
        .saturating_pow(num_operators as u32)))
        .map(|combination| {
          let base_3 = pad::PadStr::pad(
            format!("{}", radix_fmt::radix(combination, 3)).as_str(),
            num_operators,
            '0',
            pad::Alignment::Right,
            false,
          );
          (0..num_operators)
            .map(|operator| {
              let digit = base_3.chars().nth(operator).unwrap_or('0');
              if digit == '0' {
                '+'
              } else if digit == '1' {
                '*'
              } else {
                '|'
              }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

      Ok(Equation {
        solution,
        operands,
        add_mul_operators: sum_mul_operators,
        add_mul_concat_operators: sum_mul_concat_operators,
      })
    })
    .process_results(|equations| equations.collect::<Vec<_>>())?;

  let sum_add_mul = equations.iter().fold(0u64, |sum, equation| {
    let mut operands = equation.operands.clone();
    let first_operand = operands.remove(0);
    if equation.add_mul_operators.iter().any(|operators| {
      equation.solution
        == operators.iter().zip(operands.iter()).fold(
          first_operand,
          |solution, (operator, operand)| {
            if *operator == '+' {
              solution.saturating_add(*operand)
            } else if *operator == '*' {
              solution.saturating_mul(*operand)
            } else {
              solution
            }
          },
        )
    }) {
      sum.saturating_add(equation.solution)
    } else {
      sum
    }
  });

  let sum_add_mul_concat = equations.iter().try_fold(
    0u64,
    |sum, equation| -> anyhow::Result<u64> {
      let mut operands = equation.operands.clone();
      let first_operand = operands.remove(0);
      if equation
        .add_mul_concat_operators
        .iter()
        .map(|operators| -> anyhow::Result<bool> {
          Ok(
            equation.solution
              == operators.iter().zip(operands.iter()).try_fold(
                first_operand,
                |solution, (operator, operand)| -> anyhow::Result<u64> {
                  if *operator == '+' {
                    Ok(solution.saturating_add(*operand))
                  } else if *operator == '*' {
                    Ok(solution.saturating_mul(*operand))
                  } else if *operator == '|' {
                    Ok(format!("{}{}", solution, operand).parse::<u64>()?)
                  } else {
                    Ok(solution)
                  }
                },
              )?,
          )
        })
        .process_results(|results| results.into_iter().any(identity))?
      {
        Ok(sum.saturating_add(equation.solution))
      } else {
        Ok(sum)
      }
    },
  )?;

  println!("Sum (multiplication and addition): {}", sum_add_mul);
  println!(
    "Sum (multiplication, addition and concatenation): {}",
    sum_add_mul_concat
  );

  Ok(())
}

#[derive(Clone, Debug)]
struct Equation {
  solution: u64,
  operands: Vec<u64>,
  add_mul_operators: Vec<Vec<char>>,
  add_mul_concat_operators: Vec<Vec<char>>,
}
