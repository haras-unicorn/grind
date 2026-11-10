#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use std::collections::HashMap;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt").trim();

  let input_stones = input
    .split(" ")
    .map(|stone| {
      #[allow(clippy::unwrap_used, reason = "Static input")]
      let stone = stone.trim().parse::<Stone>().unwrap();
      stone
    })
    .collect::<StoneList>();
  println!("Input: {}", serialize_stones(&input_stones));

  let blink_25_stones = blink(&input_stones.clone(), 25);
  println!("Blink 25 count: {}", blink_25_stones);

  let blink_50_stones = blink(&input_stones.clone(), 50);
  println!("Blink 50 count: {}", blink_50_stones);

  let blink_75_stones = blink(&input_stones.clone(), 75);
  println!("Blink 75 count: {}", blink_75_stones);

  Ok(())
}

fn blink(stones: &StoneList, blinks: Blink) -> Stone {
  let mut cache = StoneCache::new();

  stones
    .iter()
    .map(|stone| blink_many(*stone, blinks, &mut cache))
    .sum::<u64>()
}

fn blink_many(stone: Stone, blinks: Blink, cache: &mut StoneCache) -> Stone {
  if blinks > 2 {
    if let Some(cached) = cache.get(&stone).and_then(|x| x.get(&blinks)) {
      return *cached;
    }
  }

  let mut result = 0 as Stone;
  if blinks == 1 {
    result = blink_once(stone).len() as Stone;
  } else {
    for stone in blink_once(stone).iter() {
      result = result.saturating_add(blink_many(
        *stone,
        blinks.saturating_sub(1),
        cache,
      ));
    }
  }

  if blinks > 2 {
    if let Some(cached) = cache.get_mut(&stone) {
      cached.insert(blinks, result);
    } else {
      let mut map = HashMap::new();
      map.insert(blinks, result);
      cache.insert(stone, map);
    }
  }

  result
}

fn blink_once(stone: Stone) -> StoneList {
  if stone == 0 {
    return vec![1 as Stone];
  }

  let digits = stone_digit_count(stone);
  if digits.is_multiple_of(2) {
    let lhs_stone = stone_from_digits(stone, digits.saturating_div(2), digits);
    let rhs_stone =
      stone_from_digits(stone, 0, digits.saturating_div(2).saturating_sub(1));
    return vec![lhs_stone, rhs_stone];
  }

  vec![stone.saturating_mul(2024)]
}

fn serialize_stones(stones: &StoneList) -> String {
  if stones.len() > 10 {
    let first_ten = stones
      .iter()
      .map(|stone| stone.to_string())
      .take(10)
      .join(" ");
    format!("{first_ten}...")
  } else {
    stones.iter().map(|stone| stone.to_string()).join(" ")
  }
}

type Stone = u64;
type StoneList = Vec<Stone>;
type Blink = usize;
type StoneCache = HashMap<Stone, HashMap<Blink, Stone>>;

fn stone_digit_count(stone: Stone) -> u32 {
  if stone == 0 {
    return 1;
  }

  for exp in 0..100_u32 {
    let pow = (10 as Stone).saturating_pow(exp);
    if stone.checked_div(pow).is_none_or(|x| x == 0) {
      return exp;
    }
  }

  0
}

fn stone_from_digits(stone: Stone, start: u32, end: u32) -> Stone {
  let mut result = 0 as Stone;
  for (exp, index) in (start..(end.saturating_add(1)))
    .zip(0..(end.saturating_sub(start).saturating_add(1)))
  {
    let exp_pow = (10 as Stone).saturating_pow(exp);
    #[allow(clippy::arithmetic_side_effects, reason = "Power of 10")]
    let digit = stone.saturating_div(exp_pow) % 10;
    let index_pow = (10 as Stone).saturating_pow(index);
    result = result.saturating_add(digit.saturating_mul(index_pow));
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stone_digit_count() {
    assert_eq!(stone_digit_count(0), 1);
    assert_eq!(stone_digit_count(9), 1);
    assert_eq!(stone_digit_count(10), 2);
    assert_eq!(stone_digit_count(24), 2);
    assert_eq!(stone_digit_count(12345), 5);
    assert_eq!(stone_digit_count(100000), 6);
  }

  #[test]
  fn test_stone_from_digits() {
    assert_eq!(stone_from_digits(12345, 0, 4), 12345);
    assert_eq!(stone_from_digits(12345, 0, 2), 345);
    assert_eq!(stone_from_digits(12345, 2, 4), 123);
    assert_eq!(stone_from_digits(12345, 0, 0), 5);
    assert_eq!(stone_from_digits(12345, 4, 4), 1);
  }
}
