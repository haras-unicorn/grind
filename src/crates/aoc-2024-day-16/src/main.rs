#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
  // let input = include_str!("../input.txt");

  let input = r"
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
  ";

  let map = Map::parse(input);

  println!("Input: {:?}", map);

  Ok(())
}

#[derive(Debug, Clone)]
struct Map {
  entities: Vec<Vec<Entity>>,
  #[allow(dead_code, reason = "debug")]
  height: Coordinate,
  #[allow(dead_code, reason = "debug")]
  width: Coordinate,
  start: Position,
  #[allow(dead_code, reason = "debug")]
  end: Position,
  nodes: HashMap<Head, Node>,
}

impl Map {
  fn parse(text: &str) -> Self {
    let entities = text
      .trim()
      .split('\n')
      .map(|line| line.trim().chars().map(Entity::parse).collect::<Vec<_>>())
      .collect::<Vec<_>>();
    let height = entities.len();
    let width = entities[0].len();

    let mut start = None;
    let mut end = None;
    for (y, row) in entities.iter().enumerate().take(height) {
      for (x, item) in row.iter().enumerate().take(width) {
        match item {
          Entity::Start => start = Some(Position { y, x }),
          Entity::End => end = Some(Position { y, x }),
          _ => {}
        };
      }
    }
    #[allow(clippy::unwrap_used, reason = "static input")]
    let start = start.unwrap();
    #[allow(clippy::unwrap_used, reason = "static input")]
    let end = end.unwrap();

    let mut map = Self {
      entities,
      height,
      width,
      start,
      end,
      nodes: HashMap::new(),
    };
    let nodes = map.nodes();
    map.nodes = nodes;
    map
  }

  fn nodes(&self) -> HashMap<Head, Node> {
    let mut nodes: HashMap<Head, Node> = HashMap::new();
    let mut current_nodes = Head::possibilities(self.start)
      .into_iter()
      .map(|head| Node {
        head,
        cost: START_DIRECTION.turn_cost(head.direction),
      })
      .collect::<Vec<_>>();

    while !current_nodes.is_empty() {
      let current_node = current_nodes.remove(0);
      let mut next_nodes = DIRECTIONS
        .iter()
        .cloned()
        .filter(|direction| *direction != current_node.head.direction)
        .map(|direction| current_node.turn(direction))
        .collect::<Vec<_>>();
      if let Some(node) = self.step(current_node) {
        next_nodes.push(node);
      }
      for next_node in next_nodes {
        if let Some(existing_node) = nodes.get_mut(&next_node.head) {
          *existing_node = if *existing_node < next_node {
            *existing_node
          } else {
            next_node
          };
        } else {
          nodes.insert(next_node.head, next_node);
          current_nodes.push(next_node);
        }
      }
    }

    nodes
  }

  fn get(&self, position: Position) -> Option<Entity> {
    let line = self.entities.get(position.y)?;
    let entity = line.get(position.x)?;
    Some(*entity)
  }

  fn step(&self, node: Node) -> Option<Node> {
    let offset = node.head.direction.offset();
    let position = node.head.position.apply(offset)?;
    if self.get(position) != Some(Entity::Space) {
      return None;
    }
    Some(Node {
      head: Head {
        position,
        direction: node.head.direction,
      },
      cost: node.cost.saturating_add(STEP_COST),
    })
  }
}

#[derive(Debug, Clone, Copy)]
struct Node {
  head: Head,
  cost: Cost,
}

impl Node {
  fn turn(self, direction: Direction) -> Self {
    Self {
      head: Head {
        position: self.head.position,
        direction,
      },
      cost: self
        .cost
        .saturating_add(direction.turn_cost(self.head.direction)),
    }
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.cost.eq(&other.cost)
  }
}

impl Eq for Node {}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.cost.partial_cmp(&other.cost)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Head {
  direction: Direction,
  position: Position,
}

impl Head {
  fn possibilities(position: Position) -> Vec<Head> {
    let mut heads = Vec::new();
    for direction in DIRECTIONS {
      if let Some(position) = position.apply(direction.offset()) {
        heads.push(Head {
          position,
          direction,
        });
      }
    }
    heads
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// spell-checker: disable-next-line
#[repr(u8)]
enum Direction {
  North = 0,
  East = 1,
  South = 2,
  West = 3,
}

impl Direction {
  fn turn_cost(self, other: Direction) -> Cost {
    if self == other {
      0
    } else if self.invert() == other {
      BACK_COST
    } else {
      TURN_COST
    }
  }

  fn offset(self) -> Offset {
    match self {
      Direction::North => NORTH_OFFSET,
      Direction::East => EAST_OFFSET,
      Direction::South => SOUTH_OFFSET,
      Direction::West => WEST_OFFSET,
    }
  }

  fn invert(self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }
}

const START_DIRECTION: Direction = Direction::East;

const DIRECTIONS: [Direction; 4] = [
  Direction::North,
  Direction::East,
  Direction::South,
  Direction::West,
];

const STEP_COST: Cost = 1;
const TURN_COST: Cost = 1000;
const BACK_COST: Cost = 2000;

type Cost = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  x: Coordinate,
  y: Coordinate,
}

impl Position {
  fn apply(self, offset: Offset) -> Option<Self> {
    let y = TryInto::<OffsetValue>::try_into(self.y)
      .ok()
      .and_then(|y| y.checked_add(offset.y))
      .and_then(|y| TryInto::<Coordinate>::try_into(y).ok())?;
    let x = TryInto::<OffsetValue>::try_into(self.x)
      .ok()
      .and_then(|x| x.checked_add(offset.x))
      .and_then(|x| TryInto::<Coordinate>::try_into(x).ok())?;
    Some(Self { y, x })
  }
}

type Coordinate = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset {
  x: OffsetValue,
  y: OffsetValue,
}

const NORTH_OFFSET: Offset = Offset { y: -1, x: 0 };
const EAST_OFFSET: Offset = Offset { y: 0, x: 1 };
const SOUTH_OFFSET: Offset = Offset { y: 1, x: 0 };
const WEST_OFFSET: Offset = Offset { y: 0, x: -1 };

type OffsetValue = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
  Start,
  End,
  Wall,
  Space,
}

impl Entity {
  fn parse(char: char) -> Self {
    match char {
      MAP_START_CHAR => Entity::Start,
      MAP_END_CHAR => Entity::End,
      MAP_WALL_CHAR => Entity::Wall,
      _ => Entity::Space,
    }
  }
}

const MAP_START_CHAR: char = 'S';
const MAP_END_CHAR: char = 'E';
const MAP_WALL_CHAR: char = '#';
#[allow(dead_code, reason = "nice to know")]
const MAP_SPACE_CHAR: char = '.';
