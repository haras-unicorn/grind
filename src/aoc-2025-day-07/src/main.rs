#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use itertools::Itertools;
use rayon::iter::{
  IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::{
  fmt::Display,
  str::FromStr,
  sync::atomic::{AtomicUsize, Ordering},
};

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  // let input = r"
  //   .......S.......
  //   ...............
  //   .......^.......
  //   ...............
  //   ......^.^......
  //   ...............
  //   .....^.^.^.....
  //   ...............
  //   ....^.^...^....
  //   ...............
  //   ...^.^...^.^...
  //   ...............
  //   ..^...^.....^..
  //   ...............
  //   .^.^.^.^.^...^.
  //   ...............
  // ";

  println!("Input:\n{input}");

  let mut tachyon_manifold =
    TachyonManifold::from(input.parse::<TachyonManifoldState>()?);
  tachyon_manifold.play();

  println!("\nTachyon manifold:\n{tachyon_manifold}\n");
  println!("Splits: {}", tachyon_manifold.splits());

  let mut quantum_tachyon_manifold =
    QuantumTachyonManifold::from(input.parse::<QuantumTachyonManifoldState>()?);
  quantum_tachyon_manifold.play();

  println!("\nQuantum Tachyon manifold:\n{quantum_tachyon_manifold:8}\n");
  println!("Timelines: {}", quantum_tachyon_manifold.timelines());

  Ok(())
}

#[derive(Debug, Clone)]
struct QuantumTachyonManifold {
  start: QuantumTachyonManifoldState,
  steps: Vec<QuantumTachyonManifoldStep>,
}

impl QuantumTachyonManifold {
  fn play(&mut self) {
    loop {
      let step = self
        .steps
        .last()
        .map_or(&self.start, |step| &step.next)
        .step();
      if step.beams == 0 {
        break;
      }
      self.steps.push(step);
    }
  }

  fn timelines(&self) -> usize {
    self
      .steps
      .iter()
      .map(|step| step.splits)
      .fold(0_usize, |sum, splits| sum.saturating_add(splits))
      .saturating_add(1)
  }
}

impl Display for QuantumTachyonManifold {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(width) = f.width() {
      write!(f, "Start:\n{:width$}\n", self.start)?;
      for (index, step) in self.steps.iter().enumerate() {
        write!(
          f,
          "\n\nStep: {}\nBeams: {}\nSplits: {}\nState:\n{:width$}",
          index.saturating_add(1),
          step.beams,
          step.splits,
          step.next
        )?
      }
      return Ok(());
    }
    write!(f, "Start:\n{}\n", self.start)?;
    for (index, step) in self.steps.iter().enumerate() {
      write!(
        f,
        "\n\nStep: {}\nBeams: {}\nSplits: {}\nState:\n{}",
        index.saturating_add(1),
        step.beams,
        step.splits,
        step.next
      )?
    }
    Ok(())
  }
}

impl From<QuantumTachyonManifoldState> for QuantumTachyonManifold {
  fn from(value: QuantumTachyonManifoldState) -> Self {
    Self {
      start: value,
      steps: vec![],
    }
  }
}

#[derive(Debug, Clone)]
struct TachyonManifold {
  start: TachyonManifoldState,
  steps: Vec<TachyonManifoldStep>,
}

impl TachyonManifold {
  fn play(&mut self) {
    loop {
      let step = self
        .steps
        .last()
        .map_or(&self.start, |step| &step.next)
        .step();
      if step.beams == 0 {
        break;
      }
      self.steps.push(step);
    }
  }

  fn splits(&self) -> usize {
    self
      .steps
      .iter()
      .map(|step| step.splits)
      .fold(0_usize, |sum, splits| sum.saturating_add(splits))
  }
}

impl Display for TachyonManifold {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Start:\n{}\n", self.start)?;
    for (index, step) in self.steps.iter().enumerate() {
      write!(
        f,
        "\n\nStep: {}\nBeams: {}\nSplits: {}\nState:\n{}",
        index.saturating_add(1),
        step.beams,
        step.splits,
        step.next
      )?
    }
    Ok(())
  }
}

impl From<TachyonManifoldState> for TachyonManifold {
  fn from(value: TachyonManifoldState) -> Self {
    Self {
      start: value,
      steps: vec![],
    }
  }
}

#[derive(Debug, Clone)]
struct QuantumTachyonManifoldState {
  tiles: Vec<Vec<QuantumTachyonManifoldTile>>,
}

#[derive(Debug, Clone)]
struct QuantumTachyonManifoldStep {
  splits: usize,
  beams: usize,
  next: QuantumTachyonManifoldState,
}

impl QuantumTachyonManifoldState {
  fn step(&self) -> QuantumTachyonManifoldStep {
    let splits = AtomicUsize::new(0);
    let beams = AtomicUsize::new(0);
    let tiles = self
      .tiles
      .par_iter()
      .enumerate()
      .map(|(i, line)| {
        line
          .par_iter()
          .enumerate()
          .map(|(j, tile)| match tile {
            &QuantumTachyonManifoldTile::Superposition { beam, empty } => {
              if beam != 0 || empty != 1 {
                return QuantumTachyonManifoldTile::Superposition {
                  beam,
                  empty,
                };
              }

              let mut result_beam = 0_usize;
              let mut result_empty = 0_usize;
              if i > 0
                && j < self.tiles[i].len().saturating_sub(1)
                && matches!(
                  self.tiles[i][j.saturating_add(1)],
                  QuantumTachyonManifoldTile::Deterministic(
                    DeterministicQuantumTachyonManifoldTile::Splitter
                  )
                )
              {
                if let QuantumTachyonManifoldTile::Superposition {
                  beam, ..
                } = self.tiles[i.saturating_sub(1)][j.saturating_add(1)]
                {
                  beams.fetch_add(beam, Ordering::SeqCst);
                  splits.fetch_add(beam, Ordering::SeqCst);
                  result_beam = result_beam.saturating_add(beam);
                  result_empty = result_empty.saturating_add(beam);
                }

                if matches!(
                  self.tiles[i.saturating_sub(1)][j.saturating_add(1)],
                  QuantumTachyonManifoldTile::Deterministic(
                    DeterministicQuantumTachyonManifoldTile::Start
                  )
                ) {
                  beams.fetch_add(1, Ordering::SeqCst);
                  splits.fetch_add(1, Ordering::SeqCst);
                  result_beam = result_beam.saturating_add(1);
                  result_empty = result_empty.saturating_add(1);
                }
              }

              if i > 0
                && j > 0
                && matches!(
                  self.tiles[i][j.saturating_sub(1)],
                  QuantumTachyonManifoldTile::Deterministic(
                    DeterministicQuantumTachyonManifoldTile::Splitter
                  )
                )
              {
                if let QuantumTachyonManifoldTile::Superposition {
                  beam, ..
                } = self.tiles[i.saturating_sub(1)][j.saturating_sub(1)]
                {
                  beams.fetch_add(beam, Ordering::SeqCst);
                  if j == 1 {
                    splits.fetch_add(beam, Ordering::SeqCst);
                  }
                  result_beam = result_beam.saturating_add(beam);
                  result_empty = result_empty.saturating_add(beam);
                }

                if matches!(
                  self.tiles[i.saturating_sub(1)][j.saturating_sub(1)],
                  QuantumTachyonManifoldTile::Deterministic(
                    DeterministicQuantumTachyonManifoldTile::Start
                  )
                ) {
                  beams.fetch_add(1, Ordering::SeqCst);
                  if j == 1 {
                    splits.fetch_add(1, Ordering::SeqCst);
                  }
                  result_beam = result_beam.saturating_add(1);
                  result_empty = result_empty.saturating_add(1);
                }
              }

              if i > 0
                && matches!(
                  self.tiles[i.saturating_sub(1)][j],
                  QuantumTachyonManifoldTile::Deterministic(
                    DeterministicQuantumTachyonManifoldTile::Start
                  )
                )
              {
                beams.fetch_add(1, Ordering::SeqCst);
                result_beam = result_beam.saturating_add(1);
              }

              if i > 0 {
                if let QuantumTachyonManifoldTile::Superposition {
                  beam,
                  empty,
                } = self.tiles[i.saturating_sub(1)][j]
                {
                  if beam > 0 {
                    beams.fetch_add(beam, Ordering::SeqCst);
                    result_beam = result_beam.saturating_add(beam);
                    result_empty = result_empty.saturating_add(empty);
                  }
                }
              }

              QuantumTachyonManifoldTile::Superposition {
                beam: result_beam,
                empty: result_empty.max(1),
              }
            }
            QuantumTachyonManifoldTile::Deterministic(deterministic) => {
              QuantumTachyonManifoldTile::Deterministic(*deterministic)
            }
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    QuantumTachyonManifoldStep {
      next: QuantumTachyonManifoldState { tiles },
      splits: splits.load(Ordering::SeqCst),
      beams: beams.load(Ordering::SeqCst),
    }
  }
}

impl Display for QuantumTachyonManifoldState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(width) = f.width() {
      write!(
        f,
        "{}",
        self
          .tiles
          .iter()
          .map(|line| line
            .iter()
            .map(|tile| format!("{:width$}", tile))
            .join(""))
          .join("\n"),
      )?;
      return Ok(());
    }

    write!(
      f,
      "{}",
      self
        .tiles
        .iter()
        .map(|line| line.iter().join(""))
        .join("\n"),
    )
  }
}

impl FromStr for QuantumTachyonManifoldState {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tiles = s
      .trim()
      .split('\n')
      .map(|line| {
        line
          .trim()
          .chars()
          .map(|char| {
            TachyonManifoldTile::try_from(char)
              .map(QuantumTachyonManifoldTile::from)
          })
          .collect::<Result<Vec<_>, _>>()
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self { tiles })
  }
}

#[derive(Debug, Clone)]
struct TachyonManifoldState {
  tiles: Vec<Vec<TachyonManifoldTile>>,
}

#[derive(Debug, Clone)]
struct TachyonManifoldStep {
  splits: usize,
  beams: usize,
  next: TachyonManifoldState,
}

impl TachyonManifoldState {
  fn step(&self) -> TachyonManifoldStep {
    let mut splits = 0_usize;
    let mut beams = 0_usize;
    let tiles = self
      .tiles
      .iter()
      .enumerate()
      .map(|(i, line)| {
        line
          .iter()
          .enumerate()
          .map(|(j, &tile)| {
            if !matches!(tile, TachyonManifoldTile::Empty) {
              return tile;
            }

            if i > 0
              && j < self.tiles[i].len().saturating_sub(1)
              && matches!(
                self.tiles[i.saturating_sub(1)][j.saturating_add(1)],
                TachyonManifoldTile::Start | TachyonManifoldTile::Beam
              )
              && matches!(
                self.tiles[i][j.saturating_add(1)],
                TachyonManifoldTile::Splitter
              )
            {
              beams = beams.saturating_add(1);
              splits = splits.saturating_add(1);
              return TachyonManifoldTile::Beam;
            }

            if i > 0
              && j > 0
              && matches!(
                self.tiles[i.saturating_sub(1)][j.saturating_sub(1)],
                TachyonManifoldTile::Start | TachyonManifoldTile::Beam
              )
              && matches!(
                self.tiles[i][j.saturating_sub(1)],
                TachyonManifoldTile::Splitter
              )
            {
              beams = beams.saturating_add(1);
              if j == 1 {
                splits = splits.saturating_add(1);
              }
              return TachyonManifoldTile::Beam;
            }

            if i > 0
              && matches!(
                self.tiles[i.saturating_sub(1)][j],
                TachyonManifoldTile::Start | TachyonManifoldTile::Beam
              )
            {
              beams = beams.saturating_add(1);
              return TachyonManifoldTile::Beam;
            }

            TachyonManifoldTile::Empty
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    TachyonManifoldStep {
      next: TachyonManifoldState { tiles },
      splits,
      beams,
    }
  }
}

impl Display for TachyonManifoldState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self
        .tiles
        .iter()
        .map(|line| line.iter().join(""))
        .join("\n"),
    )
  }
}

impl FromStr for TachyonManifoldState {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tiles = s
      .trim()
      .split('\n')
      .map(|line| {
        line
          .trim()
          .chars()
          .map(TachyonManifoldTile::try_from)
          .collect::<Result<Vec<_>, _>>()
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self { tiles })
  }
}

#[derive(Debug, Clone)]
enum QuantumTachyonManifoldTile {
  Deterministic(DeterministicQuantumTachyonManifoldTile),
  Superposition { beam: usize, empty: usize },
}

#[derive(Debug, Clone, Copy)]
enum DeterministicQuantumTachyonManifoldTile {
  Start,
  Splitter,
}

impl Display for QuantumTachyonManifoldTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let (Some(half_width), Some(remainder_width)) = (
      f.width().map(|width| width / 2),
      f.width().map(|width| width % 2),
    ) {
      match self {
        QuantumTachyonManifoldTile::Deterministic(deterministic) => {
          let first_pad = half_width.saturating_add(remainder_width);
          let second_pad = half_width;
          write!(f, "{deterministic:>first_pad$}{:>second_pad$}", "")
        }
        &QuantumTachyonManifoldTile::Superposition { beam, empty } => {
          if beam == 0 {
            if empty == 1 {
              let first_pad = half_width.saturating_add(remainder_width);
              let second_pad = half_width;
              return write!(
                f,
                "{TACHYON_MANIFOLD_TILE_EMPTY_CHAR:>first_pad$}{:>second_pad$}",
                ""
              );
            }
            let first_pad =
              half_width.saturating_add(remainder_width).saturating_sub(2);
            let second_pad = half_width;
            return write!(
              f,
              "{empty:>first_pad$}x{TACHYON_MANIFOLD_TILE_EMPTY_CHAR}{:>second_pad$}",
              ""
            );
          }
          let first_pad =
            half_width.saturating_add(remainder_width).saturating_sub(3);
          let second_pad = half_width.saturating_sub(2);
          write!(f, "{beam:>first_pad$}x{TACHYON_MANIFOLD_TILE_BEAM_CHAR}&{empty:>second_pad$}x{TACHYON_MANIFOLD_TILE_EMPTY_CHAR}")
        }
      }
    } else {
      match self {
        QuantumTachyonManifoldTile::Deterministic(deterministic) => {
          write!(f, "{deterministic:>4}{:>3}", "")
        }
        &QuantumTachyonManifoldTile::Superposition { beam, empty } => {
          if beam > 0 {
            if empty > 0 {
              return write!(f, "{empty}x{TACHYON_MANIFOLD_TILE_EMPTY_CHAR}&{beam}x{TACHYON_MANIFOLD_TILE_BEAM_CHAR}");
            }
            return write!(f, "{beam:>5}x{TACHYON_MANIFOLD_TILE_BEAM_CHAR}");
          }
          write!(f, "{:>7}", ".")
        }
      }
    }
  }
}

impl Display for DeterministicQuantumTachyonManifoldTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      DeterministicQuantumTachyonManifoldTile::Start => {
        TACHYON_MANIFOLD_TILE_START_STR
      }
      DeterministicQuantumTachyonManifoldTile::Splitter => {
        TACHYON_MANIFOLD_TILE_SPLITTER_STR
      }
    };

    f.pad(symbol)
  }
}

impl From<TachyonManifoldTile> for QuantumTachyonManifoldTile {
  fn from(value: TachyonManifoldTile) -> Self {
    match value {
      TachyonManifoldTile::Empty => Self::Superposition { beam: 0, empty: 1 },
      TachyonManifoldTile::Splitter => {
        Self::Deterministic(DeterministicQuantumTachyonManifoldTile::Splitter)
      }
      TachyonManifoldTile::Start => {
        Self::Deterministic(DeterministicQuantumTachyonManifoldTile::Start)
      }
      TachyonManifoldTile::Beam => Self::Superposition { beam: 1, empty: 0 },
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum TachyonManifoldTile {
  Empty,
  Splitter,
  Start,
  Beam,
}

impl Display for TachyonManifoldTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      TachyonManifoldTile::Empty => TACHYON_MANIFOLD_TILE_EMPTY_STR,
      TachyonManifoldTile::Splitter => TACHYON_MANIFOLD_TILE_SPLITTER_STR,
      TachyonManifoldTile::Start => TACHYON_MANIFOLD_TILE_START_STR,
      TachyonManifoldTile::Beam => TACHYON_MANIFOLD_TILE_BEAM_STR,
    };

    f.pad(symbol)
  }
}

impl TryFrom<char> for TachyonManifoldTile {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      TACHYON_MANIFOLD_TILE_EMPTY_CHAR => Ok(TachyonManifoldTile::Empty),
      TACHYON_MANIFOLD_TILE_SPLITTER_CHAR => Ok(TachyonManifoldTile::Splitter),
      TACHYON_MANIFOLD_TILE_START_CHAR => Ok(TachyonManifoldTile::Start),
      TACHYON_MANIFOLD_TILE_BEAM_CHAR => Ok(TachyonManifoldTile::Beam),
      _ => Err(anyhow::anyhow!("unknown tile")),
    }
  }
}

const TACHYON_MANIFOLD_TILE_EMPTY_CHAR: char = '.';
const TACHYON_MANIFOLD_TILE_SPLITTER_CHAR: char = '^';
const TACHYON_MANIFOLD_TILE_START_CHAR: char = 'S';
const TACHYON_MANIFOLD_TILE_BEAM_CHAR: char = '|';

const TACHYON_MANIFOLD_TILE_EMPTY_STR: &str = ".";
const TACHYON_MANIFOLD_TILE_SPLITTER_STR: &str = "^";
const TACHYON_MANIFOLD_TILE_START_STR: &str = "S";
const TACHYON_MANIFOLD_TILE_BEAM_STR: &str = "|";
