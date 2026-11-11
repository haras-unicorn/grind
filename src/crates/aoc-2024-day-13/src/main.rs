#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::fmt::Display;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let miscalculated_claw_machines = input
    .trim()
    .split("\n\n")
    .filter_map(|claw_machine| ClawMachine::parse(claw_machine.trim()))
    .collect::<Vec<_>>();

  let claw_machines = miscalculated_claw_machines
    .iter()
    .map(|claw_machine| claw_machine.correct())
    .collect::<Vec<_>>();

  println!("Miscalculated claw machines:");
  for claw_machine in miscalculated_claw_machines.iter() {
    println!("{}\n", claw_machine);
  }

  println!("Claw machines:");
  for claw_machine in claw_machines.iter() {
    println!("{}\n", claw_machine);
  }

  println!(
    "Miscalculated price: {}",
    miscalculated_price(&miscalculated_claw_machines)
  );

  println!("Price: {}", price(&claw_machines));

  Ok(())
}

fn price(claw_machines: &[ClawMachine]) -> Price {
  claw_machines
    .iter()
    .filter_map(|claw_machine| {
      claw_machine
        .button_a
        .presses(claw_machine.button_b, claw_machine.prize.position)
        .and_then(|button_a_presses| {
          claw_machine
            .button_b
            .presses(claw_machine.button_a, claw_machine.prize.position)
            .map(|button_b_presses| {
              button_a_presses
                .saturating_mul(claw_machine.button_a.price)
                .saturating_add(
                  button_b_presses.saturating_mul(claw_machine.button_b.price),
                )
            })
        })
    })
    .sum::<Price>()
}

fn miscalculated_price(claw_machines: &[ClawMachine]) -> Price {
  claw_machines
    .iter()
    .filter_map(|claw_machine| {
      (0..(MAX_MISCALCULATED_PRESSES + 1))
        .cartesian_product(0..(MAX_MISCALCULATED_PRESSES + 1))
        .filter(|(a_presses, b_presses)| {
          claw_machine
            .button_a
            .offset
            .mul(*a_presses)
            .add(claw_machine.button_b.offset.mul(*b_presses))
            == claw_machine.prize.position
        })
        .map(|(a_presses, b_presses)| {
          a_presses
            .saturating_mul(claw_machine.button_a.price)
            .saturating_add(
              b_presses.saturating_mul(claw_machine.button_b.price),
            )
        })
        .min()
    })
    .sum::<Price>()
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
  button_a: Button,
  button_b: Button,
  prize: Prize,
}

impl ClawMachine {
  fn parse(text: &str) -> Option<ClawMachine> {
    text.split_once("\n").and_then(|(button_a, rest)| {
      rest.split_once("\n").and_then(|(button_b, prize)| {
        Button::parse(button_a.trim()).and_then(|button_a| {
          Button::parse(button_b.trim()).and_then(|button_b| {
            Prize::parse(prize.trim()).map(|prize| Self {
              button_a,
              button_b,
              prize,
            })
          })
        })
      })
    })
  }

  fn correct(self) -> Self {
    Self {
      button_a: self.button_a,
      button_b: self.button_b,
      prize: self.prize.correct(),
    }
  }
}

impl Display for ClawMachine {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}\n{}\n{}", self.button_a, self.button_b, self.prize)
  }
}

#[derive(Debug, Clone, Copy)]
struct Prize {
  position: Position,
}

impl Prize {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(": ").and_then(|(_, position)| {
      Position::parse(position).map(|position| Self { position })
    })
  }

  fn correct(self) -> Self {
    Self {
      position: self.position.correct(),
    }
  }
}

impl Display for Prize {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Prize: {}", self.position)
  }
}

#[derive(Debug, Clone, Copy)]
struct Button {
  id: ButtonId,
  price: Price,
  offset: Position,
}

impl Button {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(": ").and_then(|(id, offset)| {
      id.split_once(" ").and_then(|(_, id)| {
        id.chars().next().and_then(|id| {
          (match id {
            'A' => Some(BUTTON_A_PRICE),
            'B' => Some(BUTTON_B_PRICE),
            _ => None,
          })
          .and_then(|price| {
            Position::parse(offset).map(|offset| Self { id, price, offset })
          })
        })
      })
    })
  }

  fn presses(self, other: Button, position: Position) -> Option<Press> {
    let dividend = position
      .y
      .value
      .saturating_mul(other.offset.x.value)
      .saturating_sub(position.x.value.saturating_mul(other.offset.y.value));
    let divisor = self
      .offset
      .y
      .value
      .saturating_mul(other.offset.x.value)
      .saturating_sub(self.offset.x.value.saturating_mul(other.offset.y.value));

    let result = (dividend as f64) / (divisor as f64);

    // spell-checker: disable-next-line
    if result.is_normal() && result.fract() == 0f64 {
      Some(result as Press)
    } else {
      None
    }
  }
}

impl Display for Button {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Button {}: {}", self.id, self.offset)
  }
}

const BUTTON_A_PRICE: Price = 3;
const BUTTON_B_PRICE: Price = 1;

const MAX_MISCALCULATED_PRESSES: Press = 100;

type ButtonId = char;
type Press = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
  x: Coordinate,
  y: Coordinate,
}

impl Position {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(", ").and_then(|(x, y)| {
      Coordinate::parse(y.trim())
        .and_then(|y| Coordinate::parse(x).map(|x| Self { y, x }))
    })
  }

  fn add(self, other: Position) -> Self {
    Self {
      x: self.x.add(other.x),
      y: self.y.add(other.y),
    }
  }

  fn mul(self, by: CoordinateValue) -> Self {
    Self {
      x: self.x.mul(by),
      y: self.y.mul(by),
    }
  }

  fn correct(self) -> Self {
    Self {
      x: self.x.correct(),
      y: self.y.correct(),
    }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}, {}", self.x, self.y)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
  axis: Axis,
  value: CoordinateValue,
  is_offset: bool,
}

impl Coordinate {
  fn parse(text: &str) -> Option<Self> {
    #[allow(clippy::unwrap_used, reason = "static input")]
    text.trim().split_once('=').map_or_else(
      || {
        text.split_once('+').and_then(|(axis, value)| {
          axis.chars().next().and_then(|axis| {
            value.trim().parse::<CoordinateValue>().ok().map(|value| {
              Coordinate {
                axis,
                value,
                is_offset: true,
              }
            })
          })
        })
      },
      |(axis, value)| {
        axis.chars().next().and_then(|axis| {
          value
            .trim()
            .parse::<CoordinateValue>()
            .ok()
            .map(|value| Coordinate {
              axis,
              value,
              is_offset: false,
            })
        })
      },
    )
  }

  fn add(self, other: Coordinate) -> Coordinate {
    Coordinate {
      axis: self.axis,
      value: self.value.saturating_add(other.value),
      is_offset: self.is_offset,
    }
  }

  fn mul(self, by: CoordinateValue) -> Coordinate {
    Coordinate {
      axis: self.axis,
      value: self.value.saturating_mul(by),
      is_offset: false,
    }
  }

  fn correct(self) -> Self {
    Self {
      axis: self.axis,
      value: self.value.saturating_add(10000000000000 as CoordinateValue),
      is_offset: self.is_offset,
    }
  }
}

impl Display for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_offset {
      write!(f, "{}+{}", self.axis, self.value)
    } else {
      write!(f, "{}={}", self.axis, self.value)
    }
  }
}

type Axis = char;
type CoordinateValue = i64;
type Price = i64;
