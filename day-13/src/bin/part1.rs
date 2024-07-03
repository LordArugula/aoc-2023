fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  input.split("\n\n")
    .map(|pattern| {
      pattern.lines()
        .map(|line| line.chars().collect())
        .collect()
    })
    .map(|pattern: Vec<Vec<_>>| {
      let height = pattern.len();
      let width = pattern[0].len();
      
      // check if pattern is mirrored across vertical
      let mut start = 1;
      while start < width {
        // check if mirror is at col_x = start
        let mut length = 1;
        while start >= length 
          && start + length - 1 < width {

          if (0..height).any(|row| {
            let a = pattern[row][start - length];
            let b = pattern[row][start + length - 1];
            a != b
          }) {
            // mirror along col_x ends here
            break;
          }
          length += 1;
        }

        if start + 1 == length || start + length > width {
          return start as u32;
        }

        start += 1;
      }
      
      // check if pattern is mirrored across horizontal
      let mut start = 1;
      while start < height {
        // check if mirror is at row_y = start
        let mut length = 1;
        while start >= length 
          && start + length - 1 < height {

          if (0..width).any(|col| {
            let a = pattern[start - length][col];
            let b = pattern[start + length - 1][col];
            a != b
          }) {
            // mirror along row_y ends here
            break;
          }
          length += 1;
        }

        if start + 1 == length || start + length > height {
          return start as u32 * 100;
        }

        start += 1;
      }
      0
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"), 405);
  }
}
