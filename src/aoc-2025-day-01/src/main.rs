#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::fmt::Display;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  // let input = r"
  //   L68
  //   L30
  //   R48
  //   L5
  //   R60
  //   L55
  //   L1
  //   L99
  //   R14
  //   L82
  // ";

  let rotations = input
    .trim()
    .split("\n")
    .map(Rotation::parse)
    .collect::<anyhow::Result<Vec<_>>>()?;

  let dial = rotations.iter().cloned().fold(Dial::new(), Dial::rotate);
  let dial_click = rotations
    .iter()
    .cloned()
    .fold(Dial::new(), Dial::rotate_click);

  // println!("Rotations:\n{}", rotations.iter().join("\n"));
  println!("Dial: {}", dial);
  println!("Dial click: {}", dial_click);

  Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Dial {
  position: u32,
  zeroes: u32,
}

impl Dial {
  fn new() -> Self {
    Self {
      position: 50,
      zeroes: 0,
    }
  }

  fn rotate(self, rotation: Rotation) -> Self {
    let position = match rotation {
      Rotation::Left(value) => {
        (self.position as i32).saturating_sub(value as i32)
      }
      Rotation::Right(value) => {
        (self.position as i32).saturating_add(value as i32)
      }
    };

    let position = position % 100;

    let position = if position < 0 {
      100_i32.saturating_add(position)
    } else {
      position
    };

    let position = position as u32;

    let zeroes = if position == 0 {
      self.zeroes.saturating_add(1)
    } else {
      self.zeroes
    };

    Self { position, zeroes }
  }

  fn rotate_click(self, rotation: Rotation) -> Self {
    let position = match rotation {
      Rotation::Left(value) => {
        (self.position as i32).saturating_sub(value as i32)
      }
      Rotation::Right(value) => {
        (self.position as i32).saturating_add(value as i32)
      }
    };

    let zeroes = if position < 0 && self.position > 0 {
      1_u32
    } else {
      0_u32
    };

    let zeroes = zeroes.saturating_add((position / 100).unsigned_abs());

    let zeroes = if position == 0 {
      zeroes.saturating_add(1)
    } else {
      zeroes
    };

    let position = position % 100;

    let position = if position < 0 {
      100_i32.saturating_add(position)
    } else {
      position
    };

    let position = position as u32;

    Self {
      position,
      zeroes: self.zeroes.saturating_add(zeroes),
    }
  }
}

impl Display for Dial {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.position, self.zeroes)
  }
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
  Left(u32),
  Right(u32),
}

impl Rotation {
  fn parse(text: &str) -> anyhow::Result<Self> {
    match text.trim().split_at(1) {
      ("L", value) => Ok(Rotation::Left(value.parse::<u32>()?)),
      ("R", value) => Ok(Rotation::Right(value.parse::<u32>()?)),
      _ => Err(anyhow::anyhow!("unknown rotation")),
    }
  }
}

impl Display for Rotation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Rotation::Left(value) => write!(f, "L{}", value),
      Rotation::Right(value) => write!(f, "R{}", value),
    }
  }
}
