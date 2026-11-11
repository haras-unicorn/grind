#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt").trim();

  let mut map = input
    .split("\n")
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|position| MapPosition {
          antenna: if position == '.' {
            None
          } else {
            Some(position)
          },
          antinode: 0,
          harmonic_antinode: 0,
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let height = map.len();
  let width = map[0].len();

  let antennas = map
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line.iter().enumerate().map(move |(x, position)| {
        position.antenna.map(|antenna| Antenna {
          position: (y, x),
          frequency: antenna,
        })
      })
    })
    .flatten()
    .collect::<Vec<_>>();

  let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
  for antenna in antennas.iter() {
    if let Some(antennas) = antenna_map.get_mut(&antenna.frequency) {
      antennas.push(antenna.clone());
    } else {
      antenna_map.insert(antenna.frequency, vec![antenna.clone()]);
    }
  }

  for antenna in antennas.iter() {
    let (y, x) = antenna.position;
    let other_antennas = antenna_map[&antenna.frequency]
      .iter()
      .filter(|other_antenna| other_antenna.position != antenna.position)
      .collect::<Vec<_>>();

    for other_antenna in other_antennas.iter() {
      let (oy, ox) = other_antenna.position;
      let distance = (
        (oy as i32).saturating_sub(y as i32),
        (ox as i32).saturating_sub(x as i32),
      );

      let antinode_distance =
        (distance.0.saturating_mul(2), distance.1.saturating_mul(2));
      let antinode_position = (
        (y as i32).saturating_add(antinode_distance.0),
        (x as i32).saturating_add(antinode_distance.1),
      );
      map.get_mut(antinode_position.0 as usize).and_then(|line| {
        line
          .get_mut(antinode_position.1 as usize)
          .map(|position| position.antinode = 1)
      });

      for mul in 1.. {
        let antinode_distance = (
          distance.0.saturating_mul(mul),
          distance.1.saturating_mul(mul),
        );
        let antinode_position = (
          (y as i32).saturating_add(antinode_distance.0),
          (x as i32).saturating_add(antinode_distance.1),
        );

        if antinode_position.0 < 0
          || antinode_position.0 >= (height as i32)
          || antinode_position.1 < 0
          || antinode_position.1 >= (width as i32)
        {
          break;
        }

        map.get_mut(antinode_position.0 as usize).and_then(|line| {
          line
            .get_mut(antinode_position.1 as usize)
            .map(|position| position.harmonic_antinode = 1)
        });
      }
    }
  }

  let antinodes = map.iter().fold(0u32, |antinodes, line| {
    antinodes.saturating_add(line.iter().fold(0u32, |antinodes, position| {
      antinodes.saturating_add(position.antinode)
    }))
  });

  let harmonic_antinodes = map.iter().fold(0u32, |harmonic_antinodes, line| {
    harmonic_antinodes.saturating_add(line.iter().fold(
      0u32,
      |harmonic_antinodes, position| {
        harmonic_antinodes.saturating_add(position.harmonic_antinode)
      },
    ))
  });

  println!(
    "Map:\n{}\nAntinodes: {:?}\n",
    serialize_map(&map, false),
    antinodes
  );
  println!(
    "Harmonic Map:\n{}\nAntinodes: {:?}",
    serialize_map(&map, true),
    harmonic_antinodes
  );

  Ok(())
}

fn serialize_map(map: &[Vec<MapPosition>], harmonic: bool) -> String {
  map
    .iter()
    .map(|line| {
      line
        .iter()
        .map(|position| match (harmonic, position) {
          (
            true,
            MapPosition {
              harmonic_antinode: 1,
              ..
            },
          )
          | (false, MapPosition { antinode: 1, .. }) => '#',
          (
            _,
            MapPosition {
              antenna: Some(antenna),
              ..
            },
          ) => *antenna,
          _ => '.',
        })
        .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join("\n")
}

#[derive(Clone, Debug)]
struct MapPosition {
  antenna: Option<char>,
  antinode: u32,
  harmonic_antinode: u32,
}

#[derive(Clone, Debug)]
struct Antenna {
  position: (usize, usize),
  frequency: char,
}
