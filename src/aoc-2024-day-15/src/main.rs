#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::fmt::Display;

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let input = Warehouse::parse(input)?;

  let mut thin_warehouse = input.clone();
  print!("Thin warehouse input:\n{thin_warehouse}\n\n");
  for _ in 1..(thin_warehouse.robot.movements.len().saturating_add(1)) {
    thin_warehouse.next()?;
  }
  print!("Thin warehouse output:\n{thin_warehouse}\n");
  print!("Thin warehouse GPS sum: {}\n\n", thin_warehouse.gps());

  let mut thick_warehouse = input.thicken();
  print!("Thick warehouse input:\n{thick_warehouse}\n\n");
  for _ in 1..(thick_warehouse.robot.movements.len().saturating_add(1)) {
    thick_warehouse.next()?;
  }
  print!("Thick warehouse output:\n{thick_warehouse}\n");
  println!("Thick warehouse GPS sum: {}", thick_warehouse.gps());

  Ok(())
}

#[derive(Debug, Clone)]
struct Warehouse {
  height: usize,
  width: usize,
  entities: Vec<Vec<Entity>>,
  #[allow(dead_code, reason = "used for debugging")]
  walls: Vec<Position>,
  boxes: Vec<Position>,
  thick_boxes: Vec<ThickPosition>,
  side_wall_thickness: WallThickness,
  robot: Robot,
  elapsed: Iteration,
}

impl Warehouse {
  fn gps(&self) -> Coordinate {
    let box_sum = self
      .boxes
      .iter()
      .map(|position| position.gps(self.side_wall_thickness))
      .sum::<Coordinate>();

    let thick_box_sum = self
      .thick_boxes
      .iter()
      .map(|position| position.gps(self.side_wall_thickness))
      .sum::<Coordinate>();

    box_sum.saturating_add(thick_box_sum)
  }

  fn next(&mut self) -> anyhow::Result<()> {
    let elapsed = self.elapsed;
    let movement = match self.robot.movements.first() {
      Some(movement) => *movement,
      None => return Err(anyhow::anyhow!("Movement unknown at {elapsed}")),
    };
    let elapsed = self.elapsed.saturating_add(1);
    self.elapsed = elapsed;
    self.robot.movements.remove(0);
    let offset = movement.to_offset();

    let next_robot_position = match self.robot.position.apply(offset) {
      Some(position) => position,
      None => return Ok(()),
    };
    let next_entity = match self.get(next_robot_position) {
      Some(entity) => entity,
      None => return Ok(()),
    };

    if next_entity == Entity::Robot {
      return Err(anyhow::anyhow!("Robot bumped into itself at {elapsed}"));
    }

    if next_entity == Entity::None {
      self.move_robot(next_robot_position)?;
      return Ok(());
    }

    if next_entity == Entity::Wall {
      return Ok(());
    }

    if next_entity == Entity::Box {
      let mut next_and_last_box_positions = Vec::new();
      let mut last_box_position = next_robot_position;
      loop {
        let next_box_position = match last_box_position.apply(offset) {
          Some(position) => position,
          None => {
            return Ok(());
          }
        };
        let next_entity = match self.get(next_box_position) {
          Some(entity) => entity,
          None => {
            return Ok(());
          }
        };

        if next_entity == Entity::Robot {
          return Err(anyhow::anyhow!("Robot bumped into itself at {elapsed}"));
        }

        if next_entity == Entity::Wall {
          return Ok(());
        }

        next_and_last_box_positions
          .push((last_box_position, next_box_position));
        if next_entity == Entity::None {
          break;
        }

        last_box_position = next_box_position;
      }

      for (last_box_position, next_box_position) in
        next_and_last_box_positions.into_iter().rev()
      {
        self.move_box(last_box_position, next_box_position)?;
      }
      self.move_robot(next_robot_position)?;
    }

    let last_thick_box_position = if next_entity == Entity::ThickBoxStart {
      ThickPosition::from_start(next_robot_position).ok_or_else(|| {
        anyhow::anyhow!(
          "Unable to construct thick box from start {next_robot_position:#}"
        )
      })?
    } else if next_entity == Entity::ThickBoxEnd {
      ThickPosition::from_end(next_robot_position).ok_or_else(|| {
        anyhow::anyhow!(
          "Unable to construct thick box from end {next_robot_position:#}"
        )
      })?
    } else {
      return Ok(());
    };
    let mut next_and_last_thick_box_positions = Vec::new();
    let mut last_thick_box_positions = vec![last_thick_box_position];
    loop {
      let mut ready_to_move = true;
      let mut next_last_thick_box_positions = Vec::new();
      for last_thick_box_position in last_thick_box_positions {
        let next_thick_box_position =
          match last_thick_box_position.apply(offset) {
            Some(position) => position,
            None => {
              return Ok(());
            }
          };
        let next_entity = match self.get_thick(next_thick_box_position) {
          Some(entities) => entities,
          None => {
            return Ok(());
          }
        };

        if next_entity.any(Entity::Robot) {
          return Err(anyhow::anyhow!("Robot bumped into itself at {elapsed}"));
        }

        if next_entity.any(Entity::Wall) {
          return Ok(());
        }

        next_and_last_thick_box_positions
          .push((last_thick_box_position, next_thick_box_position));

        if next_entity.any(Entity::ThickBoxStart)
          || next_entity.any(Entity::ThickBoxEnd)
        {
          ready_to_move = false;
        }

        if next_entity.start == Entity::ThickBoxStart {
          next_last_thick_box_positions.push(next_thick_box_position);
        } else if next_entity.start == Entity::ThickBoxEnd
          && offset != THICK_END_OFFSET
        {
          next_last_thick_box_positions.push(
            ThickPosition::from_end(next_thick_box_position.start).ok_or_else(
              || {
                anyhow::anyhow!(
                  "Unable to construct thick box from end {:#}",
                  next_thick_box_position.start
                )
              },
            )?,
          );
        }
        if next_entity.end == Entity::ThickBoxEnd {
          next_last_thick_box_positions.push(next_thick_box_position);
        } else if next_entity.end == Entity::ThickBoxStart
          && offset != THICK_START_OFFSET
        {
          next_last_thick_box_positions.push(
            ThickPosition::from_start(next_thick_box_position.end).ok_or_else(
              || {
                anyhow::anyhow!(
                  "Unable to construct thick box from start {:#}",
                  next_thick_box_position.end
                )
              },
            )?,
          );
        }
      }
      if ready_to_move {
        break;
      }
      last_thick_box_positions = next_last_thick_box_positions;
    }

    next_and_last_thick_box_positions = next_and_last_thick_box_positions
      .iter()
      .unique()
      .cloned()
      .collect::<Vec<_>>();
    for (last_thick_box_position, next_thick_box_position) in
      next_and_last_thick_box_positions.into_iter().rev()
    {
      self.move_thick_box(last_thick_box_position, next_thick_box_position)?;
    }
    self.move_robot(next_robot_position)?;

    Ok(())
  }

  fn thicken(&self) -> Self {
    let mut entities = Vec::new();
    for _ in 0..self.height {
      let mut line = Vec::new();
      for _ in 0..(self.width.saturating_mul(2)) {
        line.push(Entity::None);
      }
      entities.push(line);
    }

    let mut walls = self.walls.clone();
    for position in self.walls.iter() {
      let start_position = Position {
        y: position.y,
        x: position.x.saturating_mul(2),
      };
      let end_position = Position {
        y: start_position.y,
        x: start_position.x.saturating_add(1),
      };
      walls.push(start_position);
      entities[start_position.y][start_position.x] = Entity::Wall;
      entities[end_position.y][end_position.x] = Entity::Wall;
    }

    let mut thick_boxes = Vec::new();
    for position in self.boxes.iter() {
      let start = Position {
        y: position.y,
        x: position.x.saturating_mul(2),
      };
      let end = Position {
        y: start.y,
        x: start.x.saturating_add(1),
      };
      thick_boxes.push(ThickPosition { start, end });
      entities[start.y][start.x] = Entity::ThickBoxStart;
      entities[end.y][end.x] = Entity::ThickBoxEnd;
    }

    let robot = Robot {
      position: Position {
        y: self.robot.position.y,
        x: self.robot.position.x.saturating_mul(2),
      },
      movements: self.robot.movements.clone(),
    };
    entities[robot.position.y][robot.position.x] = Entity::Robot;

    Self {
      height: entities.len(),
      width: entities[0].len(),
      entities,
      walls,
      boxes: Vec::new(),
      thick_boxes,
      side_wall_thickness: self.side_wall_thickness.saturating_mul(2),
      robot,
      elapsed: self.elapsed,
    }
  }

  fn get(&self, position: Position) -> Option<Entity> {
    self
      .entities
      .get(position.y)
      .and_then(|line| line.get(position.x).copied())
  }

  fn get_thick(&self, position: ThickPosition) -> Option<ThickEntity> {
    self.get(position.start).and_then(|start| {
      self.get(position.end).map(|end| ThickEntity { start, end })
    })
  }

  fn move_robot(&mut self, next_position: Position) -> anyhow::Result<()> {
    if self.entities[next_position.y][next_position.x] != Entity::None {
      return Err(anyhow::anyhow!(
        "Next robot position is not none at {next_position:#}"
      ));
    }

    let last_position = self.robot.position;
    self.entities[next_position.y][next_position.x] = Entity::Robot;
    self.entities[last_position.y][last_position.x] = Entity::None;
    self.robot.position = next_position;

    Ok(())
  }

  fn move_box(
    &mut self,
    last_position: Position,
    next_position: Position,
  ) -> anyhow::Result<()> {
    if self.entities[next_position.y][next_position.x] != Entity::None {
      return Err(anyhow::anyhow!(
        "Next box position is not none at {next_position:#}"
      ));
    }

    self.entities[last_position.y][last_position.x] = Entity::None;
    self.entities[next_position.y][next_position.x] = Entity::Box;
    for position in self.boxes.iter_mut() {
      if *position == last_position {
        *position = next_position;
        break;
      }
    }

    Ok(())
  }

  fn move_thick_box(
    &mut self,
    last_position: ThickPosition,
    next_position: ThickPosition,
  ) -> anyhow::Result<()> {
    if (self.entities[next_position.start.y][next_position.start.x]
      != Entity::None
      && self.entities[next_position.start.y][next_position.start.x]
        != Entity::ThickBoxEnd)
      || (self.entities[next_position.end.y][next_position.end.x]
        != Entity::None
        && self.entities[next_position.end.y][next_position.end.x]
          != Entity::ThickBoxStart)
    {
      return Err(anyhow::anyhow!(
        "Next thick box position is not none at {next_position:#}"
      ));
    }

    self.entities[last_position.start.y][last_position.start.x] = Entity::None;
    self.entities[last_position.end.y][last_position.end.x] = Entity::None;
    self.entities[next_position.start.y][next_position.start.x] =
      Entity::ThickBoxStart;
    self.entities[next_position.end.y][next_position.end.x] =
      Entity::ThickBoxEnd;

    for position in self.thick_boxes.iter_mut() {
      if *position == last_position {
        *position = next_position;
        break;
      }
    }

    Ok(())
  }

  fn parse(text: &str) -> anyhow::Result<Self> {
    let mut entities = Vec::new();
    let mut walls = Vec::new();
    let mut boxes = Vec::new();
    let mut thick_boxes = Vec::new();
    let mut robot_position = None;
    let mut robot_movements = Vec::new();

    if let Some((map, movements)) = text.split_once("\n\n") {
      let lines = map
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>();
      for (y, line) in lines
        .iter()
        .skip(1)
        .take(lines.len().saturating_sub(2))
        .enumerate()
      {
        let mut line_entities = Vec::new();
        for (x, char) in line
          .chars()
          .skip(1)
          .take(line.len().saturating_sub(2))
          .enumerate()
        {
          line_entities.push(Entity::parse(char)?);
          match char {
            ROBOT_ENTITY_CHAR => robot_position = Some(Position { x, y }),
            WALL_ENTITY_CHAR => walls.push(Position { x, y }),
            BOX_ENTITY_CHAR => boxes.push(Position { x, y }),
            THICK_BOX_START_ENTITY_CHAR => thick_boxes.push(
              ThickPosition::from_start(Position { x, y }).ok_or_else(
                || {
                  anyhow::anyhow!(
                  "Failed to construct thick box position from start ({x}x{y})"
                )
                },
              )?,
            ),
            THICK_BOX_END_ENTITY_CHAR => {}
            NONE_ENTITY_CHAR => {}
            _ => {
              return Err(anyhow::anyhow!(
                "Unknown entity char {char:?} at ({y}x{x})"
              ))
            }
          }
        }
        entities.push(line_entities);
      }

      for (index, char) in movements
        .trim()
        .chars()
        .enumerate()
        .filter(|(_, char)| !char.is_whitespace())
      {
        robot_movements.push(
          Direction::parse(char)
            .with_context(|| format!("Failed parsing direction at {index}"))?,
        );
      }
    }
    let robot_position = robot_position
      .ok_or_else(|| anyhow::anyhow!("Robot position missing"))?;

    Ok(Self {
      height: entities.len(),
      width: entities[0].len(),
      entities,
      boxes,
      thick_boxes,
      walls,
      side_wall_thickness: 1 as WallThickness,
      robot: Robot {
        position: robot_position,
        movements: robot_movements,
      },
      elapsed: 0 as Iteration,
    })
  }
}

impl Display for Warehouse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      write!(f, "{}x{}@{:#}", self.width, self.height, self.robot)?;
    }

    for y in 0..(self.height.saturating_add(2)) {
      if y == 0 || y == self.height.saturating_add(1) {
        if y != 0 {
          writeln!(f)?;
        }
        for _ in 0..(self
          .width
          .saturating_add(self.side_wall_thickness.saturating_mul(2)))
        {
          write!(f, "{WALL_ENTITY_CHAR}")?;
        }
        continue;
      }
      let y = y.saturating_sub(1);

      writeln!(f)?;
      for _ in 0..self.side_wall_thickness {
        write!(f, "{WALL_ENTITY_CHAR}")?;
      }
      for x in 0..self.width {
        write!(f, "{}", self.entities[y][x])?;
      }
      for _ in 0..self.side_wall_thickness {
        write!(f, "{WALL_ENTITY_CHAR}")?;
      }
    }

    if !f.alternate() && !self.robot.movements.is_empty() {
      writeln!(f)?;
      for (index, movement) in self.robot.movements.iter().enumerate() {
        write!(f, "{movement}")?;
        if index
          .saturating_add(1)
          .wrapping_rem(CHARS_PER_MOVEMENT_LINE)
          == 0
        {
          writeln!(f)?;
        }
      }
    }

    Ok(())
  }
}

type Iteration = usize;
type WallThickness = usize;

const CHARS_PER_MOVEMENT_LINE: usize = 70;

#[derive(Debug, Clone)]
struct Robot {
  position: Position,
  movements: Vec<Direction>,
}

impl Display for Robot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      write!(f, "{:#}", self.position)?;
      writeln!(f)?;
    } else {
      for movement in self.movements.iter() {
        write!(f, "{movement}")?;
      }
    }

    Ok(())
  }
}

#[derive(Debug, Clone, Copy)]
struct ThickEntity {
  start: Entity,
  end: Entity,
}

impl ThickEntity {
  fn any(self, entity: Entity) -> bool {
    self.start == entity || self.end == entity
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
  Robot,
  Wall,
  Box,
  ThickBoxStart,
  ThickBoxEnd,
  None,
}

impl Entity {
  fn parse(char: char) -> anyhow::Result<Self> {
    match char {
      ROBOT_ENTITY_CHAR => Ok(Entity::Robot),
      WALL_ENTITY_CHAR => Ok(Entity::Wall),
      BOX_ENTITY_CHAR => Ok(Entity::Box),
      THICK_BOX_START_ENTITY_CHAR => Ok(Entity::ThickBoxStart),
      THICK_BOX_END_ENTITY_CHAR => Ok(Entity::ThickBoxEnd),
      NONE_ENTITY_CHAR => Ok(Entity::None),
      _ => Err(anyhow::anyhow!("Unknown entity character {char}")),
    }
  }
}

impl Display for Entity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Entity::Robot => write!(f, "{}", ROBOT_ENTITY_CHAR),
      Entity::Wall => write!(f, "{}", WALL_ENTITY_CHAR),
      Entity::Box => write!(f, "{}", BOX_ENTITY_CHAR),
      Entity::ThickBoxStart => write!(f, "{}", THICK_BOX_START_ENTITY_CHAR),
      Entity::ThickBoxEnd => write!(f, "{}", THICK_BOX_END_ENTITY_CHAR),
      Entity::None => write!(f, "{}", NONE_ENTITY_CHAR),
    }
  }
}

const ROBOT_ENTITY_CHAR: char = '@';
const WALL_ENTITY_CHAR: char = '#';
const BOX_ENTITY_CHAR: char = 'O';
const THICK_BOX_START_ENTITY_CHAR: char = '[';
const THICK_BOX_END_ENTITY_CHAR: char = ']';
const NONE_ENTITY_CHAR: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ThickPosition {
  start: Position,
  end: Position,
}

impl ThickPosition {
  fn gps(self, side_wall_thickness: WallThickness) -> Coordinate {
    let actual_y = self.start.y.saturating_add(1);
    let actual_x = self.start.x.saturating_add(side_wall_thickness);

    actual_y
      .saturating_mul(Y_GPS_COORDINATE_MULTIPLIER)
      .saturating_add(actual_x.saturating_mul(X_GPS_COORDINATE_MULTIPLIER))
  }

  fn from_end(end: Position) -> Option<Self> {
    Some(Self {
      start: end.apply(THICK_START_OFFSET)?,
      end,
    })
  }

  fn from_start(start: Position) -> Option<Self> {
    Some(Self {
      start,
      end: start.apply(THICK_END_OFFSET)?,
    })
  }

  fn apply(self, offset: Offset) -> Option<Self> {
    Some(Self {
      start: self.start.apply(offset)?,
      end: self.end.apply(offset)?,
    })
  }
}

impl Display for ThickPosition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      write!(f, "{:#}->{:#}", self.start, self.end)
    } else {
      write!(f, "{}->{}", self.start, self.end)
    }
  }
}

const THICK_END_OFFSET: Offset = Offset { x: 1, y: 0 };
const THICK_START_OFFSET: Offset = Offset { x: -1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  y: Coordinate,
  x: Coordinate,
}

impl Position {
  fn gps(self, side_wall_thickness: WallThickness) -> Coordinate {
    let actual_y = self.y.saturating_add(1);
    let actual_x = self.x.saturating_add(side_wall_thickness);

    actual_y
      .saturating_mul(Y_GPS_COORDINATE_MULTIPLIER)
      .saturating_add(actual_x.saturating_mul(X_GPS_COORDINATE_MULTIPLIER))
  }

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

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      write!(f, "({},{})", self.x, self.y)
    } else {
      write!(f, "{}", self.gps(1))
    }
  }
}

type Coordinate = usize;

const Y_GPS_COORDINATE_MULTIPLIER: Coordinate = 100;
const X_GPS_COORDINATE_MULTIPLIER: Coordinate = 1;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn parse(char: char) -> anyhow::Result<Self> {
    match char {
      UP_DIRECTION_CHAR => Ok(Self::Up),
      RIGHT_DIRECTION_CHAR => Ok(Self::Right),
      DOWN_DIRECTION_CHAR => Ok(Self::Down),
      LEFT_DIRECTION_CHAR => Ok(Self::Left),
      _ => Err(anyhow::anyhow!("Unknown direction character {char:?}")),
    }
  }

  fn to_offset(self) -> Offset {
    match self {
      Direction::Up => Offset { x: 0, y: -1 },
      Direction::Right => Offset { x: 1, y: 0 },
      Direction::Down => Offset { x: 0, y: 1 },
      Direction::Left => Offset { x: -1, y: 0 },
    }
  }
}

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Direction::Up => write!(f, "{}", UP_DIRECTION_CHAR),
      Direction::Right => write!(f, "{}", RIGHT_DIRECTION_CHAR),
      Direction::Down => write!(f, "{}", DOWN_DIRECTION_CHAR),
      Direction::Left => write!(f, "{}", LEFT_DIRECTION_CHAR),
    }
  }
}

const UP_DIRECTION_CHAR: char = '^';
const RIGHT_DIRECTION_CHAR: char = '>';
const DOWN_DIRECTION_CHAR: char = 'v';
const LEFT_DIRECTION_CHAR: char = '<';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset {
  x: OffsetValue,
  y: OffsetValue,
}

type OffsetValue = i64;
