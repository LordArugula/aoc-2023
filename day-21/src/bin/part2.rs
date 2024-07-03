fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input, 26501365);
  println!("{}", output);
}

fn part_two(input: &str, num_steps: usize) -> u64 {
  let map = input.lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let start = map.iter()
    .enumerate()
    .find_map(|(row, l)| {
      match l.iter().position(|&c| c == 'S') {
        Some(col) => Some((row, col)),
        None => None  
      }
    })
    .expect("Should have an 'S'");

  let height = map.len();
  let width = map[0].len();

  let half_height = height / 2;

  let remainder = num_steps % height;
  println!("{remainder}");
  let radius = ((num_steps - half_height) - if remainder > half_height { 1 } else { 0 }).div_ceil(height);

  let min_radius = radius.min(if remainder % 2 != 0 {5} else {6});
  let min_num_steps = (min_radius * height + remainder).min(num_steps);
  
  // println!("{}", min_num_steps);
  
  let offset_height = min_radius * height;
  let offset_width = min_radius * width;
  
  let mut open_space = vec![(start.0 + offset_height, start.1 + offset_width)];
  let mut next_spaces = std::collections::HashSet::new();
  let mut visited = std::collections::HashSet::new();
  let mut record = std::collections::HashSet::<(usize, usize)>::new();

  let mut num_steps_taken = 0;
  while num_steps_taken < min_num_steps {
    if num_steps_taken % 2 == num_steps % 2 {
      record.extend(next_spaces.iter());
    }
    open_space.extend(next_spaces.iter());
    next_spaces.clear();

    while let Some((row, col)) = open_space.pop() {
      if visited.contains(&(row, col)) {
        continue;
      }

      visited.insert((row, col));
      
      let remapped_row = row % map.len();
      let remapped_col = col % map[remapped_row].len();

      let height = map.len();
      let width = map[remapped_row].len();

      if map[(remapped_row + 1) % height][remapped_col] != '#' {
        next_spaces.insert((row + 1, col));
      }

      if map[remapped_row][(col + 1) % width] != '#' {
        next_spaces.insert((row, col + 1));
      }

      if map[(row - 1) % height][remapped_col] != '#' {
        next_spaces.insert((row - 1, col));
      }

      if map[remapped_row][(col - 1) % width] != '#' {
        next_spaces.insert((row, col - 1));
      }
    }

    num_steps_taken += 1;
  }
  
  if num_steps < 1000 {
    record.extend(next_spaces.iter());
  }
  
  if radius < 5 {
    return record.len() as u64;
  } 

  let row_length = min_radius * 2;
  let col_length = min_radius * 2;
  let mid_col = min_radius;
  let mid_row = min_radius;

  let top_center = count_steps(&map, &record, (0, mid_col));
  let top_center_left = count_steps(&map, &record, (0, mid_col - 1));
  let top_center_right = count_steps(&map, &record, (0, mid_col + 1));
  let top_center_below_one = count_steps(&map, &record, (1, mid_col));
  let top_center_below_one_left = count_steps(&map, &record, (1, mid_col - 1));
  let top_center_below_one_right = count_steps(&map, &record, (1, mid_col + 1));
  let top_center_below_one_left_two = count_steps(&map, &record, (1, mid_col - 2));
  let top_center_below_one_right_two = count_steps(&map, &record, (1, mid_col + 2));
  println!("   {} {} {}", top_center_left, top_center, top_center_right);
  println!("{} {} {} {} {}", top_center_below_one_left_two, top_center_below_one_left, top_center_below_one, top_center_below_one_right, top_center_below_one_right_two);
  println!();

  let bottom_center = count_steps(&map, &record, (row_length, mid_col));
  let bottom_center_left = count_steps(&map, &record, (row_length, mid_col - 1));
  let bottom_center_right = count_steps(&map, &record, (row_length, mid_col + 1));
  let bottom_center_above_one = count_steps(&map, &record, (row_length - 1, mid_col));
  let bottom_center_above_one_left = count_steps(&map, &record, (row_length - 1, mid_col - 1));
  let bottom_center_above_one_right = count_steps(&map, &record, (row_length - 1, mid_col + 1));
  let bottom_center_above_one_left_two = count_steps(&map, &record, (row_length - 1, mid_col - 2));
  let bottom_center_above_one_right_two = count_steps(&map, &record, (row_length - 1, mid_col + 2));
  let bottom_center_above_two_right_one = count_steps(&map, &record, (row_length - 2, mid_col + 1));
  println!("         {}", bottom_center_above_two_right_one);
  println!("{} {} {} {} {}", bottom_center_above_one_left_two, bottom_center_above_one_left, bottom_center_above_one, bottom_center_above_one_right, bottom_center_above_one_right_two);
  println!("   {} {} {}", bottom_center_left, bottom_center, bottom_center_right);
  println!();
  
  let left_center = count_steps(&map, &record, (mid_row, 0));
  let left_center_above = count_steps(&map, &record, (mid_row - 1, 0));
  let left_center_below = count_steps(&map, &record, (mid_row + 1, 0));
  let left_center_right_one = count_steps(&map, &record, (mid_row, 1));
  let left_center_right_one_above = count_steps(&map, &record, (mid_row - 1, 1));
  let left_center_right_one_below = count_steps(&map, &record, (mid_row - 1, 1));
  let left_center_right_one_above_two = count_steps(&map, &record, (mid_row - 2, 1));
  let left_center_right_one_below_two = count_steps(&map, &record, (mid_row + 2, 1));
  println!("   {}", left_center_right_one_above_two);
  println!("{} {}", left_center_above, left_center_right_one_above);
  println!("{} {}", left_center, left_center_right_one);
  println!("{} {}", left_center_below, left_center_right_one_below);
  println!("   {}", left_center_right_one_below_two);
  
  let right_center = count_steps(&map, &record, (mid_row, col_length));
  let right_center_above = count_steps(&map, &record, (mid_row - 1, col_length));
  let right_center_below = count_steps(&map, &record, (mid_row + 1, col_length));
  let right_center_left_one = count_steps(&map, &record, (mid_row, col_length - 1));
  let right_center_left_one_above = count_steps(&map, &record, (mid_row - 1, col_length - 1));
  let right_center_left_one_below = count_steps(&map, &record, (mid_row - 1, col_length - 1));
  let right_center_left_one_above_two = count_steps(&map, &record, (mid_row - 2, col_length - 1));
  let right_center_left_one_below_two = count_steps(&map, &record, (mid_row + 2, col_length - 1));
  println!("{}", right_center_left_one_above_two);
  println!("{} {}", right_center_left_one_above, right_center_above);
  println!("{} {}", right_center_left_one, right_center);
  println!("{} {}", right_center_left_one_below, right_center_below);
  println!("{}", right_center_left_one_below_two);

  let center = count_steps(&map, &record, (mid_row, mid_col));
  let center_above = count_steps(&map, &record, (mid_row - 1, mid_col));
  println!("{} {}", center, center_above);

  let k = (num_steps / height) as u64;
  let n1 = center * (k * (k - 1) - 2);
  let n2 = center_above * (k * (k - 2) + 1); 
  let n3 = (k - 1) * (top_center_below_one_left + top_center_below_one_right + bottom_center_above_one_left + bottom_center_above_one_right);
  println!("{} {} {} {} {}", k - 1, top_center_below_one_left, top_center_below_one_right, bottom_center_above_one_left, bottom_center_above_one_right);
  let n4 = (k - 2) * bottom_center_above_two_right_one;
  let n5 = top_center + top_center_below_one 
    + bottom_center + bottom_center_above_one
    + left_center + left_center_right_one
    + right_center + right_center_left_one;
  let n6 = k * (top_center_left + top_center_right + bottom_center_left + bottom_center_right);
  println!("{} {} {} {} {} {}", n1, n2, n3, n4, n5, n6);
  
  let total_steps = n1 + n2 + n3 + n4 + n5 + n6;
  
  // (0..((min_radius * 2 + 1) * height))
  //   .for_each(|row| {
  //     (0..((min_radius * 2 + 1) * width))
  //       .for_each(|col| {
  //         if record.contains(&(row, col)) {
  //           print!("O");
  //         } else {
  //           print!("{}", map[row % height][col % width]);
  //         }
  //       });
  //     println!();
  //   });
  
  // println!("{:?}", record);

  // if num_steps_taken < num_steps {
  //   println!("{}", num_steps_taken);
  // }

  total_steps
}

fn count_steps(
  map: &Vec<Vec<char>>,
  record: &std::collections::HashSet<(usize, usize)>,
  (big_row, big_col): (usize, usize)
) -> u64 {
  let mut i = 0;
  let height = map.len();
  let width = map[0].len();
  for row in (big_row * height)..((big_row + 1) * height) {
    for col in (big_col * width)..((big_col + 1) * width) {
      if record.contains(&(row, col)) {
        i += 1;
      }
    }
  }
  i
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
  
  #[test]
  fn test_part_two_6_steps() {
    assert_eq!(part_two(INPUT, 6), 16);
  }
  
  #[test]
  fn test_part_two_10_steps() {
    assert_eq!(part_two(INPUT, 10), 50);
  }
  
  #[test]
  fn test_part_two_33_steps() {
    assert_eq!(part_two(INPUT, 33), 644);
  }
  
  #[test]
  fn test_part_two_50_steps() {
    assert_eq!(part_two(INPUT, 50), 1594);
  }
  
  #[test]
  fn test_part_two_55_steps() {
    assert_eq!(part_two(INPUT, 55), 1914);
  }
  
  #[test]
  fn test_part_two_65_steps() {
    assert_eq!(part_two(INPUT, 65), 2722);
  }
  
  #[test]
  fn test_part_two_77_steps() {
    assert_eq!(part_two(INPUT, 77), 3836);
  }
  
  #[test]
  fn test_part_two_100_steps() {
    assert_eq!(part_two(INPUT, 100), 6536);
  }
  
  #[test]
  fn test_part_two_500_steps() {
    assert_eq!(part_two(INPUT, 500), 167004);
  }
  
  #[test]
  fn test_part_two_1000_steps() {
    assert_eq!(part_two(INPUT, 1000), 668697);
  }
  
  #[test]
  fn test_part_two_5000_steps() {  
    assert_eq!(part_two(INPUT, 5000), 16733044);
  }
}
