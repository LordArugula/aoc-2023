fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let engine_schematic = input.lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut part_number_sum = 0;
  
  let mut row_index = 0;
  while row_index < engine_schematic.len() {
    let mut col_index = 0;
    while col_index < engine_schematic[row_index].len() {
      if !engine_schematic[row_index][col_index].is_digit(10) {
        col_index += 1;
        continue;
      }
      
      let mut end_idx = col_index;
      while end_idx < engine_schematic[row_index].len() && engine_schematic[row_index][end_idx].is_digit(10) {
        end_idx += 1;
      }
      
      let part_num = engine_schematic[row_index][col_index..end_idx]
        .iter()
        .fold(0, |val, c| val * 10 + c.to_digit(10).expect("Should be a number"));

      let search_row_begin_idx = row_index.saturating_sub(1);
      let search_row_end_idx = std::cmp::min(row_index + 1, engine_schematic.len() - 1);
      
      let search_col_begin_idx = col_index.saturating_sub(1);
      let search_col_end_idx = std::cmp::min(end_idx, engine_schematic[row_index].len() - 1);
      
      let is_adjacent_symbol = (search_row_begin_idx..=search_row_end_idx).any(|row| {
        (search_col_begin_idx..=search_col_end_idx).any(|col| {
          match engine_schematic[row][col] {
            '0'..='9' => false,
            '.' => false,
            _ => true
          }
        })
      });

      if is_adjacent_symbol {
        part_number_sum += part_num;
      }
      col_index = end_idx;
    }
    row_index += 1;
  }

  part_number_sum
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
      assert_eq!(part_one("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 4361);
  }
}
