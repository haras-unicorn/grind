#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

fn main() -> anyhow::Result<()> {
  let example = r"
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
  ";
  println!("Example:\n{example}\n\n");

  let example_factory = example.parse::<Factory>()?;
  println!("Example factory:\n{example_factory}\n\n");

  let example_configure_presses = example_factory.configure_presses();
  println!("Example configure presses:\n{example_configure_presses}\n\n");

  let example_configure_presses_with_joltage =
    example_factory.configure_presses_with_joltage();
  println!("Example configure presses with joltage:\n{example_configure_presses_with_joltage}\n\n");

  // let input = include_str!("../input.txt");
  // println!("Input:\n{input}\n\n");

  // let input_factory = input.parse::<Factory>()?;
  // println!("Input factory:\n{input_factory}\n\n");

  // let input_configure_presses = input_factory.configure_presses();
  // println!("Input configure presses:\n{input_configure_presses}\n\n");

  Ok(())
}

#[derive(Debug, Clone)]
struct Factory(Vec<Machine>);

impl Factory {
  fn machines(&self) -> &Vec<Machine> {
    &self.0
  }

  fn configure_presses(&self) -> usize {
    self.machines().iter().fold(0_usize, |sum, mahcine| {
      sum.saturating_add(mahcine.configure_presses())
    })
  }

  fn configure_presses_with_joltage(&self) -> usize {
    self.machines().iter().fold(0_usize, |sum, mahcine| {
      sum.saturating_add(mahcine.configure_presses_with_joltage())
    })
  }
}

impl Display for Factory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.machines().into_iter().join("\n"))
  }
}

impl FromStr for Factory {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let machines = s
      .trim()
      .split('\n')
      .map(|line| line.trim().parse::<Machine>())
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self(machines))
  }
}

#[derive(Debug, Clone)]
struct Machine {
  indicator_lights: Vec<IndicatorLight>,
  buttons: Vec<Button>,
  joltage_requirements: Vec<JoltageRequirement>,
}

impl Machine {
  fn configure_presses(&self) -> usize {
    for presses in (0..(1 << self.buttons.len()))
      .map(Presses)
      .sorted_by_key(|presses| presses.num_on())
    {
      let mut indicator_lights = (0..self.indicator_lights.len())
        .map(|_| IndicatorLight::Off)
        .collect::<Vec<_>>();

      for button in self
        .buttons
        .iter()
        .enumerate()
        .filter(|&(index, _)| presses.is_on(index))
        .map(|(_, &button)| button)
      {
        for index in button.indices() {
          indicator_lights[index] = indicator_lights[index].switch();
        }
      }

      if indicator_lights == self.indicator_lights {
        return presses.num_on();
      }
    }

    0
  }

  fn configure_presses_with_joltage(&self) -> usize {
    for presses in (0..(1 << self.buttons.len()))
      .map(Presses)
      .sorted_by_key(|presses| presses.num_on())
    {
      let mut joltage = self
        .joltage_requirements
        .iter()
        .map(|_| 0 as JoltageRequirement)
        .collect::<Vec<_>>();

      let mut indicator_lights = (0..self.indicator_lights.len())
        .map(|_| IndicatorLight::Off)
        .collect::<Vec<_>>();

      for button in self
        .buttons
        .iter()
        .enumerate()
        .filter(|&(index, _)| presses.is_on(index))
        .map(|(_, &button)| button)
      {
        for index in button.indices() {
          indicator_lights[index] = indicator_lights[index].switch();
          joltage[index] = joltage[index].saturating_add(1)
        }
      }

      if indicator_lights == self.indicator_lights
        && joltage == self.joltage_requirements
      {
        return presses.num_on();
      }
    }

    0
  }
}

#[derive(Debug, Clone, Copy)]
struct Presses(u64);

impl Presses {
  fn num_on(&self) -> usize {
    self.0.count_ones() as usize
  }

  fn is_on(&self, index: usize) -> bool {
    self.0 & (1 << index) != 0
  }
}

impl Display for Machine {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}] {} {{{}}}",
      self.indicator_lights.iter().join(""),
      self.buttons.iter().join(" "),
      self.joltage_requirements.iter().join(",")
    )
  }
}

impl FromStr for Machine {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut sections = s.trim().split(' ').collect::<Vec<_>>();
    let Some(indicator_lights) = sections.first() else {
      return Err(anyhow::anyhow!("empty machine {s}"));
    };
    let indicator_lights = indicator_lights
      .chars()
      .skip(1)
      .take(indicator_lights.len().saturating_sub(2))
      .map(<IndicatorLight as TryFrom<char>>::try_from)
      .collect::<Result<Vec<_>, _>>()?;
    sections.remove(0);
    let Some(joltage_requirements) = sections.pop() else {
      return Err(anyhow::anyhow!("empty machine {s}"));
    };
    let joltage_requirements = joltage_requirements
      [1..joltage_requirements.len().saturating_sub(1)]
      .split(',')
      .map(|joltage_requirement| {
        joltage_requirement.parse::<JoltageRequirement>()
      })
      .collect::<Result<Vec<_>, _>>()?;

    let buttons = sections
      .iter()
      .map(|section| section.parse::<Button>())
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self {
      indicator_lights,
      buttons,
      joltage_requirements,
    })
  }
}

#[derive(Debug, Clone, Copy)]
struct Button(u64);

impl Button {
  fn indices(&self) -> impl Iterator<Item = IndicatorLightIndex> {
    let val = self.0;
    (0..64)
      .filter(move |i| val & (1 << i) != 0)
      .map(|i| i as IndicatorLightIndex)
  }
}

impl Display for Button {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({})", self.indices().into_iter().join(","))
  }
}

impl FromStr for Button {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Button(
      s[1..s.len().saturating_sub(1)]
        .split(',')
        .fold(Some(0_u64), |sum, next| {
          Some(sum?.saturating_add(1 << next.parse::<u64>().ok()?))
        })
        .ok_or_else(|| anyhow::anyhow!("error parsing button"))?,
    ))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndicatorLight {
  On,
  Off,
}

impl IndicatorLight {
  fn switch(&self) -> Self {
    match self {
      IndicatorLight::On => IndicatorLight::Off,
      IndicatorLight::Off => IndicatorLight::On,
    }
  }
}

impl Display for IndicatorLight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      IndicatorLight::On => write!(f, "{INDICATOR_LIGHT_ON_CHAR}"),
      IndicatorLight::Off => write!(f, "{INDICATOR_LIGHT_OFF_CHAR}"),
    }
  }
}

impl TryFrom<char> for IndicatorLight {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      INDICATOR_LIGHT_ON_CHAR => Ok(IndicatorLight::On),
      INDICATOR_LIGHT_OFF_CHAR => Ok(IndicatorLight::Off),
      _ => Err(anyhow::anyhow!("unknown indicator light symbol {value}")),
    }
  }
}

const INDICATOR_LIGHT_ON_CHAR: char = '#';
const INDICATOR_LIGHT_OFF_CHAR: char = '.';

type IndicatorLightIndex = usize;

type JoltageRequirement = u64;
