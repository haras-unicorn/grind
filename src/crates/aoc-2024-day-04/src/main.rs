#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let parsed = input
    .split("\n")
    .filter(|x| !x.is_empty())
    .map(|line| line.trim().chars().collect::<Vec<_>>())
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>();

  let height = parsed.len();
  let width = parsed[0].len();

  let xmas = (0..width)
    .map(|x| {
      (0..height)
        .map(|y| {
          if parsed[y][x] == 'X' {
            let north: usize = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x))
                .copied(),
              parsed
                .get(y.wrapping_sub(2))
                .and_then(|row| row.get(x))
                .copied(),
              parsed
                .get(y.wrapping_sub(3))
                .and_then(|row| row.get(x))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let northeast = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_sub(2))
                .and_then(|row| row.get(x.wrapping_add(2)))
                .copied(),
              parsed
                .get(y.wrapping_sub(3))
                .and_then(|row| row.get(x.wrapping_add(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let east = if [
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_add(2)))
                .copied(),
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_add(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let southeast = if [
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(2))
                .and_then(|row| row.get(x.wrapping_add(2)))
                .copied(),
              parsed
                .get(y.wrapping_add(3))
                .and_then(|row| row.get(x.wrapping_add(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let south = if [
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x))
                .copied(),
              parsed
                .get(y.wrapping_add(2))
                .and_then(|row| row.get(x))
                .copied(),
              parsed
                .get(y.wrapping_add(3))
                .and_then(|row| row.get(x))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let southwest = if [
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(2))
                .and_then(|row| row.get(x.wrapping_sub(2)))
                .copied(),
              parsed
                .get(y.wrapping_add(3))
                .and_then(|row| row.get(x.wrapping_sub(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let west = if [
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_sub(2)))
                .copied(),
              parsed
                .get(y)
                .and_then(|row| row.get(x.wrapping_sub(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };
            let northwest = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y.wrapping_sub(2))
                .and_then(|row| row.get(x.wrapping_sub(2)))
                .copied(),
              parsed
                .get(y.wrapping_sub(3))
                .and_then(|row| row.get(x.wrapping_sub(3)))
                .copied(),
            ] == [Some('M'), Some('A'), Some('S')]
            {
              1
            } else {
              0
            };

            north
              .saturating_add(northeast)
              .saturating_add(east)
              .saturating_add(southeast)
              .saturating_add(south)
              .saturating_add(southwest)
              .saturating_add(west)
              .saturating_add(northwest)
          } else {
            0
          }
        })
        .sum::<usize>()
    })
    .sum::<usize>();

  let x_mas = (0..width)
    .map(|x| {
      (0..height)
        .map(|y| {
          if parsed[y][x] == 'A' {
            let s_up: usize =
              if [
                parsed
                  .get(y.wrapping_sub(1))
                  .and_then(|row| row.get(x.wrapping_sub(1)))
                  .copied(),
                parsed
                  .get(y.wrapping_sub(1))
                  .and_then(|row| row.get(x.wrapping_add(1)))
                  .copied(),
                parsed
                  .get(y.wrapping_add(1))
                  .and_then(|row| row.get(x.wrapping_add(1)))
                  .copied(),
                parsed
                  .get(y.wrapping_add(1))
                  .and_then(|row| row.get(x.wrapping_sub(1)))
                  .copied(),
              ] == [Some('S'), Some('S'), Some('M'), Some('M')]
              {
                1
              } else {
                0
              };

            let s_right = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
            ] == [Some('M'), Some('S'), Some('S'), Some('M')]
            {
              1
            } else {
              0
            };

            let s_down = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
            ] == [Some('M'), Some('M'), Some('S'), Some('S')]
            {
              1
            } else {
              0
            };

            let s_left = if [
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
              parsed
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied(),
              parsed
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied(),
            ] == [Some('S'), Some('M'), Some('M'), Some('S')]
            {
              1
            } else {
              0
            };

            s_up
              .saturating_add(s_right)
              .saturating_add(s_down)
              .saturating_add(s_left)
          } else {
            0
          }
        })
        .sum::<usize>()
    })
    .sum::<usize>();

  println!("XMAS: {:?}", xmas);

  println!("X-MAS: {:?}", x_mas);

  Ok(())
}
