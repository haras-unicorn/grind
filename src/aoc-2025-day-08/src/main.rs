#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::{collections::HashSet, fmt::Display, str::FromStr};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let example = r"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
  ";
  println!("Example:\n{example}\n\n");

  let example_playground = example.parse::<Playground>()?;
  println!("{example_playground}\n\n");

  let example_circuits = example_playground.circuits(10, 3);
  println!("{example_circuits}");

  let input = include_str!("../input.txt");
  println!("Input:\n{input}\n\n");

  let input_playground = input.parse::<Playground>()?;
  println!("{input_playground}\n\n");

  let input_circuits = input_playground.circuits(1000, 3);
  println!("{input_circuits}");

  let complete_input_circuit = input_playground.complete_circuit();
  println!("{complete_input_circuit}");

  Ok(())
}

#[derive(Debug, Clone)]
struct Playground {
  boxes: Vec<JunctionBox>,
  pairs: Vec<JunctionBoxPair>,
}

impl Playground {
  fn complete_circuit(&self) -> CompleteCircuitSolution {
    let mut circuits = self
      .boxes
      .iter()
      .map(|&r#box| Circuit::from(r#box))
      .collect::<Vec<_>>();
    let mut last = Option::<JunctionBoxPair>::None;

    for pair in self.pairs.iter() {
      let Some((lhs_index, lhs_circuit)) = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.boxes.contains(&pair.lhs))
      else {
        continue;
      };
      let Some((rhs_index, rhs_circuit)) = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.boxes.contains(&pair.rhs))
      else {
        continue;
      };
      if lhs_index != rhs_index {
        let result_circuit = lhs_circuit
          .boxes
          .iter()
          .chain(rhs_circuit.boxes.iter())
          .cloned()
          .collect::<Circuit>();

        circuits.remove(lhs_index);
        let Some((rhs_index, _)) = circuits
          .iter()
          .enumerate()
          .find(|(_, circuit)| circuit.boxes.contains(&pair.rhs))
        else {
          continue;
        };
        circuits.remove(rhs_index);
        circuits.push(result_circuit);

        last = Some(*pair);
        if circuits.len() == 1 {
          break;
        }
      }
    }

    let Some(last) = last else {
      return CompleteCircuitSolution::default();
    };

    let Some(circuit) = circuits.first().cloned() else {
      return CompleteCircuitSolution::default();
    };

    let value = last.lhs.x.saturating_mul(last.rhs.x);

    CompleteCircuitSolution {
      circuit,
      last,
      value: value as Solution,
    }
  }

  fn circuits(&self, pairs: usize, top: usize) -> CircuitSolution {
    let mut circuits = self
      .boxes
      .iter()
      .map(|&r#box| Circuit::from(r#box))
      .collect::<Vec<_>>();

    for pair in self.pairs.iter().take(pairs) {
      let Some((lhs_index, lhs_circuit)) = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.boxes.contains(&pair.lhs))
      else {
        continue;
      };
      let Some((rhs_index, rhs_circuit)) = circuits
        .iter()
        .enumerate()
        .find(|(_, circuit)| circuit.boxes.contains(&pair.rhs))
      else {
        continue;
      };
      if lhs_index != rhs_index {
        let result_circuit = lhs_circuit
          .boxes
          .iter()
          .chain(rhs_circuit.boxes.iter())
          .cloned()
          .collect::<Circuit>();

        circuits.remove(lhs_index);
        let Some((rhs_index, _)) = circuits
          .iter()
          .enumerate()
          .find(|(_, circuit)| circuit.boxes.contains(&pair.rhs))
        else {
          continue;
        };
        circuits.remove(rhs_index);
        circuits.push(result_circuit);
      }
    }

    circuits.sort_by_key(|circuit| circuit.boxes.len());
    circuits.reverse();

    let top = circuits.iter().take(top).cloned().collect::<Vec<_>>();
    let value = top
      .iter()
      .map(|circuit| circuit.boxes.len())
      .fold(1 as Solution, |solution, circuit_len| {
        solution.saturating_mul(circuit_len)
      });

    CircuitSolution {
      circuits,
      top,
      value,
    }
  }
}

impl Display for Playground {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\nJunction boxes:\n")?;
    for r#box in self.boxes.iter() {
      writeln!(f, "{}", r#box)?;
    }
    write!(f, "\n\nJunction box pairs:\n\n")?;
    for pair in self.pairs.iter() {
      write!(f, "{}\n\n", pair)?;
    }
    Ok(())
  }
}

impl FromStr for Playground {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let boxes = s
      .trim()
      .split('\n')
      .map(|junction_box| junction_box.parse::<JunctionBox>())
      .collect::<Result<Vec<_>, _>>()?;

    let mut pairs = boxes
      .iter()
      .enumerate()
      .flat_map(|(index, &r#box)| {
        boxes
          .iter()
          .skip(index.saturating_add(1))
          .filter(move |&&inner| inner != r#box)
          .map(move |&inner| JunctionBoxPair::new(r#box, inner))
      })
      .collect::<Vec<_>>();
    pairs.sort_by(|lhs, rhs| lhs.dist.total_cmp(&rhs.dist));

    Ok(Self { boxes, pairs })
  }
}

#[derive(Debug, Clone, Default)]
struct CompleteCircuitSolution {
  circuit: Circuit,
  last: JunctionBoxPair,
  value: Solution,
}

impl Display for CompleteCircuitSolution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Circuit: {}\n\n", self.circuit)?;

    let last = self.last;
    write!(f, "Last: {last}\n\n")?;

    let value = self.value;
    write!(f, "Value: {value}")?;

    Ok(())
  }
}

#[derive(Debug, Clone)]
struct CircuitSolution {
  circuits: Vec<Circuit>,
  top: Vec<Circuit>,
  value: Solution,
}

impl Display for CircuitSolution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let circuits_display = Circuit::display(self.circuits.iter());
    write!(f, "Circuits: {}\n\n", circuits_display)?;

    let top_display = Circuit::display(self.top.iter());
    write!(f, "Top: {}\n\n", top_display)?;

    write!(f, "Value: {}", self.value)?;

    Ok(())
  }
}

#[derive(Debug, Clone, Default)]
struct Circuit {
  boxes: HashSet<JunctionBox>,
}

impl Circuit {
  fn display<'a, T: Iterator<Item = &'a Circuit>>(mut circuits: T) -> String {
    circuits.join("\n")
  }
}

impl Display for Circuit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.boxes.iter().join(" <-> "))
  }
}

impl From<JunctionBox> for Circuit {
  fn from(value: JunctionBox) -> Self {
    let mut boxes = HashSet::new();
    boxes.insert(value);
    Self { boxes }
  }
}

impl FromIterator<JunctionBox> for Circuit {
  fn from_iter<T: IntoIterator<Item = JunctionBox>>(iter: T) -> Self {
    let mut boxes = HashSet::new();
    for r#box in iter.into_iter() {
      boxes.insert(r#box);
    }
    Self { boxes }
  }
}

#[derive(Debug, Clone, Copy, Default)]
struct JunctionBoxPair {
  lhs: JunctionBox,
  rhs: JunctionBox,
  dist: Distance,
}

impl JunctionBoxPair {
  fn new(lhs: JunctionBox, rhs: JunctionBox) -> Self {
    let sum =
      Distance::powf(rhs.x as Distance - lhs.x as Distance, 2 as Distance)
        + Distance::powf(rhs.y as Distance - lhs.y as Distance, 2 as Distance)
        + Distance::powf(rhs.z as Distance - lhs.z as Distance, 2 as Distance);

    let dist = Distance::sqrt(sum);
    if dist == 0.0 {
      println!("sqrt({rhs} - {lhs}) = sqrt({sum}) = {dist}");
    }
    Self { lhs, rhs, dist }
  }
}

impl PartialEq for JunctionBoxPair {
  fn eq(&self, other: &Self) -> bool {
    self.lhs == other.lhs && self.rhs == other.rhs
  }
}

impl Eq for JunctionBoxPair {}

impl std::hash::Hash for JunctionBoxPair {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.lhs.hash(state);
    self.rhs.hash(state);
  }
}

impl Display for JunctionBoxPair {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Lhs: {}\nRhs: {}\nDist: {}",
      self.lhs, self.rhs, self.dist
    )
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct JunctionBox {
  x: Coordinate,
  y: Coordinate,
  z: Coordinate,
}

impl Display for JunctionBox {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{COORDINATE_SEPARATOR}{}{COORDINATE_SEPARATOR}{}",
      self.x, self.y, self.z
    )
  }
}

impl FromStr for JunctionBox {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut result = JunctionBox::default();

    let coordinates = s
      .trim()
      .split(COORDINATE_SEPARATOR)
      .map(|coordinate| coordinate.parse::<Coordinate>())
      .collect::<Result<Vec<_>, _>>()?;

    if let Some(&x) = coordinates.first() {
      result.x = x;
    }
    if let Some(&y) = coordinates.get(1) {
      result.y = y;
    }
    if let Some(&z) = coordinates.get(2) {
      result.z = z;
    }

    Ok(result)
  }
}

type Solution = usize;

type Distance = f64;

type Coordinate = u64;

const COORDINATE_SEPARATOR: char = ',';
