fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 { 
  let engine_schematic = input.lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let potential_gears = input.lines()
    .enumerate()
    .map(|(idx, l)| {
      l.char_indices()
        .filter(|&(_, c)| {
          c == '*'
        })
        .map(move |(char_idx, _)| {
          (idx, char_idx) 
        })
    })
    .flatten();

  let mut row_index = 0;
  let mut nums : Vec<(usize, usize, usize, u32)> = Vec::new();
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
      nums.push((row_index, col_index, end_idx - 1, part_num));
      col_index = end_idx;
    }
    row_index += 1;
  }
  
  let total_gear_ratios = potential_gears
    .map(|(row_idx, col_idx)| {
      let row_begin_idx = row_idx.saturating_sub(1);
      let row_end_idx = std::cmp::min(row_idx + 1, engine_schematic.len() - 1);
      
      let col_begin_idx = col_idx.saturating_sub(1);
      let col_end_idx = std::cmp::min(col_idx + 1, engine_schematic[row_idx].len() - 1);

      let nearby_numbers = nums.iter()
        .filter(|(row, col, col_end, num)| {
          return !(*row < row_begin_idx || *row > row_end_idx 
            || *col_end <col_begin_idx || *col > col_end_idx);
        })
        .collect::<Vec<_>>();

      if nearby_numbers.len() >= 2 {
        let gear_ratio = 
          nearby_numbers.iter().fold(1, |val, num| val * num.3);
        return gear_ratio;
      }
      
      return 0;
    })
    .sum();

  return total_gear_ratios;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 467835);
  }
}
