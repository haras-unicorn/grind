#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

fn main() -> anyhow::Result<()> {
  let input = include_str!("../input.txt").trim();

  let mut input_blocks = Vec::new();
  for (i, char) in input.chars().enumerate() {
    #[allow(clippy::unwrap_used, reason = "")]
    let size = char.to_digit(10).unwrap();
    for _ in 0..size {
      if i % 2 == 0 {
        input_blocks.push(Some(i / 2));
      } else {
        input_blocks.push(None);
      }
    }
  }

  let mut fragmented_output_blocks = input_blocks.clone();
  let free_block_indices = fragmented_output_blocks
    .iter()
    .enumerate()
    .filter(|(_, block)| block.is_none())
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  let taken_block_indices = fragmented_output_blocks
    .iter()
    .enumerate()
    .rev()
    .filter(|(_, block)| block.is_some())
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  for (free_block_index, taken_block_index) in
    free_block_indices.iter().zip(taken_block_indices.iter())
  {
    if free_block_index > taken_block_index {
      break;
    }

    fragmented_output_blocks.swap(*free_block_index, *taken_block_index);
  }

  let mut defragmented_output_blocks = input_blocks.clone();
  let mut free_block_index_groups = Vec::new();
  for (index, block) in defragmented_output_blocks.iter().enumerate() {
    if block.is_none() {
      if index == 0
        || defragmented_output_blocks[index.saturating_sub(1)]
          != defragmented_output_blocks[index]
      {
        free_block_index_groups.push(vec![index]);
      } else if let Some(last) = free_block_index_groups.last_mut() {
        last.push(index);
      }
    }
  }
  let mut taken_block_index_groups = Vec::new();
  for (index, block) in defragmented_output_blocks.iter().enumerate() {
    if block.is_some() {
      if index == 0
        || defragmented_output_blocks[index.saturating_sub(1)]
          != defragmented_output_blocks[index]
      {
        taken_block_index_groups.push(vec![index]);
      } else if let Some(last) = taken_block_index_groups.last_mut() {
        last.push(index);
      }
    }
  }
  for free_block_index_group in free_block_index_groups.iter() {
    let mut free_space = free_block_index_group.len();
    while let Some((
      last_fitting_taken_block_index_group_index,
      last_fitting_taken_block_index_group,
    )) = taken_block_index_groups
      .clone()
      .iter()
      .enumerate()
      .rev()
      .find(|(_, taken_index_group)| {
        taken_index_group[0] > free_block_index_group[0]
          && taken_index_group.len() <= free_space
      })
    {
      for (taken_index, free_index) in
        last_fitting_taken_block_index_group.iter().zip(
          free_block_index_group
            .iter()
            .skip(free_block_index_group.len().saturating_sub(free_space)),
        )
      {
        defragmented_output_blocks.swap(*taken_index, *free_index);
      }
      taken_block_index_groups
        .remove(last_fitting_taken_block_index_group_index);
      free_space =
        free_space.saturating_sub(last_fitting_taken_block_index_group.len());
    }
  }

  let fragmented_sum = fragmented_output_blocks
    .iter()
    .enumerate()
    .filter_map(|(index, block)| block.map(|block| (index, block)))
    .map(|(index, size)| index.saturating_mul(size))
    .sum::<usize>();

  let defragmented_sum = defragmented_output_blocks
    .iter()
    .enumerate()
    .filter_map(|(index, block)| block.map(|block| (index, block)))
    .map(|(index, size)| index.saturating_mul(size))
    .sum::<usize>();

  println!("Input:               {}", serialize_blocks(&input_blocks));
  println!(
    "Fragmented output:   {}",
    serialize_blocks(&fragmented_output_blocks)
  );
  println!(
    "Defragmented output: {}",
    serialize_blocks(&defragmented_output_blocks)
  );

  println!("Fragmented sum:      {}", fragmented_sum);
  println!("Defragmented sum:    {}", defragmented_sum);

  Ok(())
}

fn serialize_blocks(blocks: &[Option<usize>]) -> String {
  blocks
    .iter()
    .map(|block| match block {
      Some(block) => {
        #[allow(clippy::unwrap_used, reason = "Modulo used")]
        let digit = char::from_digit((block % 10) as u32, 10).unwrap();
        digit
      }
      None => '.',
    })
    .collect::<String>()
}
