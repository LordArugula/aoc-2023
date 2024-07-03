fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 {
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
      let mut num_allowed_mistakes = 1;
      while start >= length 
        && start + length - 1 < width {

        let num_mistakes = (0..height)
          .filter(|&row| {
            let a = pattern[row][start - length];
            let b = pattern[row][start + length - 1];
            a != b
          })
          .count();
      
        if num_mistakes > num_allowed_mistakes {
          // mirror along col_x ends here
          break;
        } else {
          num_allowed_mistakes -= num_mistakes;
        }
        length += 1;
      }

      if (start + 1 == length || start + length > width)
        && num_allowed_mistakes == 0 {
        return start as u32;
      }

      start += 1;
    }

    // check if pattern is mirrored across horizontal
    let mut start = 1;
    while start < height {
      // check if mirror is at row_y = start
      let mut length = 1;
      let mut num_allowed_mistakes = 1;
      while start >= length 
        && start + length - 1 < height {

        let num_mistakes = (0..width)
          .filter(|&col| {
            let a = pattern[start - length][col];
            let b = pattern[start + length - 1][col];
            a != b
          })
          .count();
        if num_mistakes > num_allowed_mistakes {
          // mirror along row_y ends here
          break;
        } else {
          num_allowed_mistakes -= num_mistakes;
        }
        length += 1;
      }

      if (start + 1 == length || start + length > height)
        && num_allowed_mistakes == 0 {
        return start as u32 * 100;
      }

      start += 1;
    }
    0
  })
  // .map(|(pattern, vertical, horizontal)| {
  //   println!("{:?} {:?}", vertical, horizontal);
  //   let height = pattern.len();
  //   let width = pattern[0].len();

  //   // check if pattern is mirrored across horizontal
  //   let mut start = 1;
  //   while start < height {
  //     // check if mirror is at row_y = start
  //     let mut length = 1;
  //     // the number of mistakes we can have
  //     let mut allowed_mistakes = 1;
  //     while start >= length 
  //       && start + length - 1 < height {

  //       let num_mistakes = (0..width)
  //         .filter(|&col| {
  //           let a = pattern[start - length][col];
  //           let b = pattern[start + length - 1][col];
  //           a != b
  //         })
  //         .count();

  //       if num_mistakes > 0 {
  //         if num_mistakes > allowed_mistakes {
  //           // mirror along row_y ends here
  //           break;
  //         } else {
  //           allowed_mistakes -= 1;
  //         }
  //       }
  //       length += 1;
  //     }

  //     if (start + 1 == length || start + length > width) 
  //     && allowed_mistakes == 0 {
  //       println!("{} {} {}", start, length, start * 100);
  //       return start as u32 * 100;
  //     }

  //     start += 1;
  //   }

  //   // check if pattern is mirrored across vertical
  //   let mut start = 1;
  //   while start < width {
  //     // check if mirror is at col_x = start
  //     let mut length = 1;
  //     // the number of mistakes we can have
  //     let mut allowed_mistakes = 1;
  //     while start >= length 
  //       && start + length - 1 < width {

  //       let num_mistakes = (0..height)
  //         .filter(|&row| {
  //           let a = pattern[row][start - length];
  //           let b = pattern[row][start + length - 1];
  //           a != b
  //         })
  //         .count();
                
  //       if num_mistakes > 0 {
  //         if num_mistakes > allowed_mistakes {
  //           // mirror along col_x ends here
  //           break;
  //         } else {
  //           allowed_mistakes -= 1;
  //         }
  //       }
        
  //       length += 1;
  //     }

  //     if (start + 1 == length || start + length > width) 
  //       && allowed_mistakes == 0 {
  //       println!("{} {}, {}", start, length, start);
  //       return start as u32;
  //     }

  //     start += 1;
  //   }

  //   0
  // })
  .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("#.##..##.
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
#....#..#"), 400);
  }
}
