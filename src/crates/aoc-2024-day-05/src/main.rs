#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let mut split = input.split("\n\n");

  #[allow(clippy::unwrap_used, reason = "Static input")]
  let rules = split
    .next()
    .unwrap()
    .trim()
    .split("\n")
    .map(|rule| {
      let mut split = rule.split("|");
      #[allow(clippy::unwrap_used, reason = "Static input")]
      let result = (
        split.next().unwrap().trim().to_owned(),
        split.next().unwrap().trim().to_owned(),
      );
      result
    })
    .collect::<Vec<_>>();

  #[allow(clippy::unwrap_used, reason = "Static input")]
  let updates = split
    .next()
    .unwrap()
    .trim()
    .split("\n")
    .map(|update| {
      update
        .split(",")
        .map(|page| page.trim().to_owned())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let ordered_middle_sum = updates
    .iter()
    .filter(|update| {
      rules
        .iter()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .all(|rule| {
          update.iter().position(|page| page == &rule.0)
            < update.iter().position(|page| page == &rule.1)
        })
    })
    .map(|update| {
      #[allow(clippy::unwrap_used, reason = "Static input")]
      let middle = update[update.len() / 2].parse::<u32>().unwrap();
      middle
    })
    .sum::<u32>();

  let unordered_ordered_middle_sum = updates
    .iter()
    .filter(|update| {
      !rules
        .iter()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .all(|rule| {
          update.iter().position(|page| page == &rule.0)
            < update.iter().position(|page| page == &rule.1)
        })
    })
    .map(|update| {
      let relevant_rules = rules
        .iter()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .collect::<Vec<_>>();

      let mut ordered = update.clone();
      ordered.sort_by(|a, b| {
        let rule = relevant_rules.iter().find(|rule| {
          (&rule.0 == a || &rule.1 == a) && (&rule.0 == b || &rule.1 == b)
        });
        match rule {
          Some(rule) => {
            if &rule.0 == a {
              std::cmp::Ordering::Less
            } else {
              std::cmp::Ordering::Greater
            }
          }
          None => std::cmp::Ordering::Equal,
        }
      });

      ordered
    })
    .map(|update| {
      #[allow(clippy::unwrap_used, reason = "Static input")]
      let middle = update[update.len() / 2].parse::<u32>().unwrap();
      middle
    })
    .sum::<u32>();

  println!("Ordered middle sum: {}", ordered_middle_sum);
  println!(
    "Unordered ordered middle sum: {}",
    unordered_ordered_middle_sum
  );

  Ok(())
}
