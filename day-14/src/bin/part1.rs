fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let mut platform: Vec<Vec<_>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

  let mut row_idx = 1;
  while row_idx < platform.len() {
    let mut col_idx = 0;
    while col_idx < platform[row_idx].len() {
      if let 'O' = platform[row_idx][col_idx] {
        // try to move north
        let mut move_up_idx = row_idx;
        while move_up_idx > 0 {
          match platform[move_up_idx - 1][col_idx] {
            '.' => move_up_idx -= 1,
            _ => break,
          }
        }
        platform[row_idx][col_idx] = '.';
        platform[move_up_idx][col_idx] = 'O';
      }
      
      col_idx += 1; 
    }
    
    row_idx += 1;
  }
  println!();

  platform.iter()
    .enumerate()
    .map(|(row_idx, row)| {
      let num_rocks = row.iter()
        .filter(|&&c| c == 'O')
        .count();
      (platform.len() - row_idx) * num_rocks
    })
    .sum::<usize>() as u32
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."), 136);
  }
}
