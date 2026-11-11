#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt");

  let reports = input
    .split("\n")
    .filter(|numbers| !numbers.is_empty())
    .map(|report| {
      #[allow(clippy::unwrap_used, reason = "Static input works")]
      report
        .split(" ")
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let safe = reports
    .iter()
    .filter(|report| {
      (report.is_sorted_by(|a, b| a >= b) || report.is_sorted_by(|a, b| a <= b))
        && (report
          .iter()
          .take(report.len().saturating_sub(1))
          .zip(report.iter().skip(1))
          .all(|(x, y)| {
            let diff = x.abs_diff(*y);
            (1..=3).contains(&diff)
          }))
    })
    .count();

  let tolerant = reports
    .iter()
    .filter(|report| {
      report.iter().enumerate().any(|(i, _)| {
        let mut r#try = (*report).clone();
        r#try.remove(i);
        (r#try.is_sorted_by(|a, b| a >= b) || r#try.is_sorted_by(|a, b| a <= b))
          && (r#try
            .iter()
            .take(r#try.len().saturating_sub(1))
            .zip(r#try.iter().skip(1))
            .all(|(x, y)| {
              let diff = x.abs_diff(*y);
              (1..=3).contains(&diff)
            }))
      })
    })
    .count();

  println!("Safe {}", safe);
  println!("Tolerant {}", tolerant);

  Ok(())
}
