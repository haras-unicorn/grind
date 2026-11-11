#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use colored::Colorize;
use std::{collections::HashMap, fmt::Display, io::Write};

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");
  let max_y: Coordinate = 102;
  let max_x: Coordinate = 100;
  let safety_factor_seconds = 100 as Second;

  let area = Area::parse(input, max_y, max_x);
  println!("Area: \n{area}\n{area:#}\n");

  calculate_safety_factor(area.clone(), safety_factor_seconds)?;

  search_for_christmas_tree(area.clone())?;

  Ok(())
}

fn calculate_safety_factor(
  mut area: Area,
  seconds: Second,
) -> anyhow::Result<()> {
  area.scrub(seconds as VelocityValue);
  let safety_factor = area.safety_factor();
  println!("Area at {seconds}s:");
  print!("{area}\n{area:#}\n");
  print!("Safety factor: {safety_factor}\n\n");

  Ok(())
}

fn search_for_christmas_tree(area: Area) -> anyhow::Result<()> {
  for second in (0 as VelocityValue).. {
    let mut area = area.clone();
    area.scrub(
      second
        .saturating_mul(area.max_y.saturating_add(1) as VelocityValue)
        .saturating_add(12),
    );
    println!("Area at {second}s:");
    print!("{area}\n{area:#}\n");
    print!("Is it a christmas tree? (y/n): ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    println!();
    if input == "y\n" {
      println!("Elapsed {}s", area.elapsed);
      break;
    }
  }

  Ok(())
}

type Second = usize;

#[derive(Debug, Clone, Eq)]
struct Area {
  robots: Vec<Robot>,
  max_y: Coordinate,
  max_x: Coordinate,
  elapsed: VelocityValue,
}

impl Area {
  fn parse(text: &str, max_y: Coordinate, max_x: Coordinate) -> Area {
    let robots = text
      .trim()
      .split('\n')
      .filter(|line| !line.starts_with("//"))
      .filter_map(|robot| Robot::parse(robot.trim()))
      .collect::<Vec<_>>();

    Self {
      robots,
      max_y,
      max_x,
      elapsed: 0,
    }
  }

  fn repeats_at(&self) -> Second {
    self
      .max_x
      .saturating_add(1)
      .saturating_mul(self.max_y.saturating_add(1)) as Second
  }

  fn scrub(&mut self, by: VelocityValue) {
    self
      .robots
      .iter_mut()
      .for_each(|robot| *robot = robot.scrub(by, self.max_y, self.max_x));
    self.elapsed = self.elapsed.saturating_add(by);
  }

  fn safety_factor(&self) -> SafetyFactor {
    self.counts_by_quadrant().values().fold(
      1 as SafetyFactor,
      |safety_factor, robot_count| -> usize {
        safety_factor.saturating_mul(*robot_count)
      },
    )
  }

  fn counts_by_quadrant(&self) -> HashMap<Quadrant, Count> {
    let mut quadrants: HashMap<Quadrant, SafetyFactor> = HashMap::new();
    for quadrant in self
      .robots
      .iter()
      .filter_map(|robot| robot.position.quadrant(self.max_y, self.max_x))
    {
      if let Some(robot_count) = quadrants.get_mut(&quadrant) {
        *robot_count = robot_count.saturating_add(1 as SafetyFactor);
      } else {
        quadrants.insert(quadrant, 1 as SafetyFactor);
      }
    }
    quadrants
  }

  fn counts_by_position(&self) -> HashMap<Position, Count> {
    let mut counts: HashMap<Position, usize> = HashMap::new();
    for robot in self.robots.iter() {
      if let Some(robot_count) = counts.get_mut(&robot.position) {
        *robot_count = robot_count.saturating_add(1);
      } else {
        counts.insert(robot.position, 1usize);
      }
    }
    counts
  }
}

impl Display for Area {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      let half_y = self.max_y.saturating_div(2);
      let half_x = self.max_x.saturating_div(2);
      let counts_by_position = self.counts_by_position();
      for y in 0..(self.max_y.saturating_add(1)) {
        for x in 0..(self.max_x.saturating_add(1)) {
          let mut to_write = ".".to_string();
          let position = Position { x, y };
          if let Some(count) = counts_by_position.get(&position) {
            to_write = count.to_string();
          }
          if x == half_x || y == half_y {
            write!(f, "{}", to_write.red())?;
          } else {
            write!(f, "{}", to_write)?;
          }
        }

        if y != self.max_y {
          writeln!(f)?;
        }
      }

      Ok(())
    } else {
      writeln!(
        f,
        "({}x{})⟳{}?{}@{}",
        self.max_x.saturating_add(1),
        self.max_y.saturating_add(1),
        self.repeats_at(),
        self.robots.len(),
        self.elapsed
      )?;

      // for (index, robot) in self.robots.iter().enumerate() {
      //   write!(f, "{}", robot)?;
      //   if index != self.robots.len().saturating_sub(1) {
      //     write!(f, "\n")?;
      //   }
      // }

      Ok(())
    }
  }
}

impl PartialEq for Area {
  fn eq(&self, other: &Self) -> bool {
    self.robots == other.robots
      && self.max_y == other.max_y
      && self.max_x == other.max_x
  }
}

type SafetyFactor = usize;
type Count = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
  position: Position,
  velocity: Velocity,
}

impl Robot {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(' ').and_then(|(position, velocity)| {
      position.split_once('=').and_then(|(_, position)| {
        Position::parse(position).and_then(|position| {
          velocity.split_once('=').and_then(|(_, velocity)| {
            Velocity::parse(velocity)
              .map(|velocity| Self { position, velocity })
          })
        })
      })
    })
  }

  fn scrub(
    self,
    by: VelocityValue,
    max_y: Coordinate,
    max_x: Coordinate,
  ) -> Self {
    Self {
      position: self.position.mul(self.velocity, by, max_y, max_x),
      velocity: self.velocity,
    }
  }
}

impl Display for Robot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "p={} v={}", self.position, self.velocity)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  y: Coordinate,
  x: Coordinate,
}

impl Position {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(",").and_then(|(x, y)| {
      x.parse::<Coordinate>()
        .ok()
        .and_then(|x| y.parse::<Coordinate>().ok().map(|y| Self { y, x }))
    })
  }

  fn quadrant(self, max_y: Coordinate, max_x: Coordinate) -> Option<Quadrant> {
    let half_x = max_x.saturating_div(2);
    let half_y = max_y.saturating_div(2);

    if self.x < half_x && self.y < half_y {
      return Some(Quadrant::Northwest);
    } else if self.x > half_x && self.y < half_y {
      return Some(Quadrant::Northeast);
    } else if self.x > half_x && self.y > half_y {
      return Some(Quadrant::Southeast);
    } else if self.x < half_x && self.y > half_y {
      return Some(Quadrant::Southwest);
    }

    None
  }

  fn mul(
    self,
    velocity: Velocity,
    by: VelocityValue,
    max_y: Coordinate,
    max_x: Coordinate,
  ) -> Self {
    self.add(velocity.mul(by), max_y, max_x)
  }

  fn add(
    self,
    velocity: Velocity,
    max_y: Coordinate,
    max_x: Coordinate,
  ) -> Self {
    Self {
      y: Self::wrap(
        (self.y as VelocityValue).saturating_add(velocity.y),
        max_y,
      ),
      x: Self::wrap(
        (self.x as VelocityValue).saturating_add(velocity.x),
        max_x,
      ),
    }
  }

  fn wrap(velocity: VelocityValue, max: Coordinate) -> Coordinate {
    let len = max.saturating_add(1);
    if velocity < 0 {
      let len = len as VelocityValue;
      let result = len.saturating_add(velocity.overflowing_rem(len).0);
      if result == len {
        0 as Coordinate
      } else {
        result as Coordinate
      }
    } else {
      (velocity as Coordinate).overflowing_rem(len).0
    }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

type Coordinate = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
  Northwest,
  Northeast,
  Southeast,
  Southwest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velocity {
  y: VelocityValue,
  x: VelocityValue,
}

impl Velocity {
  fn parse(text: &str) -> Option<Self> {
    text.split_once(",").and_then(|(x, y)| {
      x.parse::<VelocityValue>()
        .ok()
        .and_then(|x| y.parse::<VelocityValue>().ok().map(|y| Self { y, x }))
    })
  }

  fn mul(self, by: VelocityValue) -> Self {
    Self {
      x: self.x.saturating_mul(by),
      y: self.y.saturating_mul(by),
    }
  }
}

impl Display for Velocity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

type VelocityValue = i64;
