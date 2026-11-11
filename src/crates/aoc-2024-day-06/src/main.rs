#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::collections::HashSet;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let map = input
    .split("\n")
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|object| MapPosition {
          object,
          previous_guard_directions: HashSet::new(),
        })
        .collect::<Vec<_>>()
    })
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  let mut walk_map = map.clone();
  guard_walk(&mut walk_map, true)?;

  let visited = walk_map
    .iter()
    .flatten()
    .filter(|position| position.object == 'X')
    .count()
    .saturating_add(1);

  let mut loop_map = map.clone();
  find_loops(&mut loop_map)?;
  let loops = loop_map
    .iter()
    .flatten()
    .filter(|position| position.object == 'O')
    .count();

  println!("Visited: {}", visited);
  println!("Loops: {}", loops);

  Ok(())
}

type Map = Vec<Vec<MapPosition>>;

#[derive(Debug, Clone)]
struct MapPosition {
  object: char,
  previous_guard_directions: HashSet<char>,
}

fn find_loops(map: &mut Map) -> anyhow::Result<()> {
  let height = map.len();
  let width = map[0].len();

  let loop_map = map.clone();
  let possible_positions = (0..height)
    .cartesian_product(0..width)
    .filter(|(y, x)| {
      let object = loop_map[*y][*x].object;
      object != '#'
        && object != '^'
        && object != '>'
        && object != 'v'
        && object != '<'
    })
    .collect::<Vec<_>>();
  possible_positions
    .iter()
    .enumerate()
    .map(|(i, (y, x))| {
      println!(
        "Finding loop ({}/{}) at: ({},{})",
        i,
        possible_positions.len(),
        y,
        x
      );
      let mut alternative_map = loop_map.clone();
      alternative_map[*y][*x].object = '#';
      guard_walk(&mut alternative_map, false)
        .map(|loop_found| (y, x, loop_found))
    })
    .process_results(|iter| {
      iter
        .filter(|(_, _, loop_found)| *loop_found)
        .map(|(y, x, _)| (y, x))
        .for_each(|(y, x)| map[*y][*x].object = 'O')
    })?;

  Ok(())
}

fn guard_walk(map: &mut Map, print: bool) -> anyhow::Result<bool> {
  let height = map.len();
  let width = map[0].len();

  let mut guard_position = (0..height)
    .cartesian_product(0..width)
    .find(|(y, x)| ['^', '>', 'v', '<'].contains(&map[*y][*x].object))
    .ok_or(anyhow::anyhow!("Guard not found"))?;
  let mut guard_direction = match map[guard_position.0][guard_position.1].object
  {
    '^' => (-1, 0),
    '>' => (0, 1),
    'v' => (1, 0),
    '<' => (0, -1),
    _ => {
      return Err(anyhow::anyhow!("Invalid guard position"));
    }
  };

  let mut guard_facing_exit = match (guard_position, guard_direction) {
    ((0, _), (-1, 0)) => true,
    ((_, 0), (0, -1)) => true,
    ((y_pos, _), (1, 0)) if y_pos == height => true,
    ((_, x_pos), (0, 1)) if x_pos == width => true,
    _ => false,
  };
  let mut stuck_in_loop = map[guard_position.0][guard_position.1]
    .previous_guard_directions
    .contains(&map[guard_position.0][guard_position.1].object);
  while !guard_facing_exit && !stuck_in_loop {
    let previous_guard_position = guard_position;
    let attempted_guard_position = (
      (guard_position.0 as i32).saturating_add(guard_direction.0) as usize,
      (guard_position.1 as i32).saturating_add(guard_direction.1) as usize,
    );

    let guard_facing_obstacle = map
      .get(attempted_guard_position.0)
      .and_then(|line| line.get(attempted_guard_position.1))
      .map(|position| position.object)
      == Some('#');
    if guard_facing_obstacle {
      guard_direction = match guard_direction {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => {
          return Err(anyhow::anyhow!("Invalid guard direction"));
        }
      }
    } else {
      guard_position = attempted_guard_position;
    }

    let guard_direction_object = match guard_direction {
      (-1, 0) => '^',
      (0, 1) => '>',
      (1, 0) => 'v',
      (0, -1) => '<',
      _ => {
        return Err(anyhow::anyhow!("Invalid guard direction"));
      }
    };

    guard_facing_exit = match (guard_position, guard_direction) {
      ((0, _), (-1, 0)) => true,
      ((_, 0), (0, -1)) => true,
      ((y_pos, _), (1, 0)) if y_pos == height.saturating_sub(1) => true,
      ((_, x_pos), (0, 1)) if x_pos == width.saturating_sub(1) => true,
      _ => false,
    };
    stuck_in_loop = map[guard_position.0][guard_position.1]
      .previous_guard_directions
      .contains(&guard_direction_object);

    map[previous_guard_position.0][previous_guard_position.1].object = 'X';
    map[guard_position.0][guard_position.1].object = match guard_direction {
      (-1, 0) => '^',
      (0, 1) => '>',
      (1, 0) => 'v',
      (0, -1) => '<',
      _ => {
        return Err(anyhow::anyhow!("Invalid guard direction"));
      }
    };
    map[guard_position.0][guard_position.1]
      .previous_guard_directions
      .insert(guard_direction_object);
    if print {
      println!(
        "Map ({}x{}):\n{}\nGuard position: {:?}\nGuard direction: {:?}\nFacing exit?: {}\n",
        height,
        width,
        serialize_map(map),
        guard_position,
        guard_direction,
        guard_facing_exit
      );
    }
  }

  Ok(stuck_in_loop)
}

fn serialize_map(map: &Map) -> String {
  map
    .iter()
    .map(|line| {
      line
        .iter()
        .map(|position| position.object)
        .collect::<String>()
    })
    .collect::<Vec<String>>()
    .join("\n")
}
