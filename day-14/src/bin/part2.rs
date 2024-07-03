fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
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
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
  let mut col_idx = 1;
  while col_idx < platform[0].len() {
    let mut row_idx = 0;
    while row_idx < platform.len() {
      if let 'O' = platform[row_idx][col_idx] {
        // try to move west
        let mut move_left_idx = col_idx;
        while move_left_idx > 0 {
          match platform[row_idx][move_left_idx - 1] {
            '.' => move_left_idx -= 1,
            _ => break,
          }
        }
        platform[row_idx][col_idx] = '.';
        platform[row_idx][move_left_idx] = 'O';
      }

      row_idx += 1;
    }

    col_idx += 1; 
  }
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
  let mut row_idx = platform.len() - 2;
  loop {
    let mut col_idx = 0;
    while col_idx < platform[row_idx].len() {
      if let 'O' = platform[row_idx][col_idx] {
        // try to move south
        let mut move_down_idx = row_idx;
        while move_down_idx < platform.len() - 1 {
          match platform[move_down_idx + 1][col_idx] {
            '.' => move_down_idx += 1,
            _ => break,
          }
        }
        platform[row_idx][col_idx] = '.';
        platform[move_down_idx][col_idx] = 'O';
      }

      col_idx += 1; 
    }

    if row_idx == 0 {
      break;
    }
    row_idx -= 1;
  }
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
  let mut col_idx = platform[0].len() - 2;
  loop {
    let mut row_idx = 0;
    while row_idx < platform.len() {
      if let 'O' = platform[row_idx][col_idx] {
        // try to move east
        let mut move_right_idx = col_idx;
        while move_right_idx < platform[row_idx].len() - 1 {
          match platform[row_idx][move_right_idx + 1] {
            '.' => move_right_idx += 1,
            _ => break,
          }
        }
        platform[row_idx][col_idx] = '.';
        platform[row_idx][move_right_idx] = 'O';
      }

      row_idx += 1;
    }

    if col_idx == 0 {
      break;
    }
    col_idx -= 1; 
  }
}

fn part_two(input: &str) -> u32 {
  let mut platform: Vec<Vec<_>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

  let iterations = 1_00;
  let mut idx = 0;

  // TODO detect cycle
  // let mut load_sequence = Vec::new();
  // let mut cycle_idx: Option<u32> = None;

  while idx < iterations {
    tilt_north(&mut platform);
    tilt_west(&mut platform);
    tilt_south(&mut platform);
    tilt_east(&mut platform);
    let load = platform.iter()
      .enumerate()
      .map(|(row_idx, row)| {
        let num_rocks = row.iter()
          .filter(|&&c| c == 'O')
          .count();
        (platform.len() - row_idx) * num_rocks
      })
      .sum::<usize>() as u32;

    // match load_sequence.iter().rposition(|&x| x == load) {
    //   Some(mut idx) => {
    //     while idx > 0 {

    //       // look at next element
          
    //       idx -= 1;
    //     }
    //   },
    //   None => todo!(),
    // }

    // load_sequence.push(load);
    
    println!("{load}");
    idx += 1;
  }
  
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
  fn test_part_two() {
    assert_eq!(part_two("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."), 64);
  }
}
