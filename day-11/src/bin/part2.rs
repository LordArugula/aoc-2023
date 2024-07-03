mod galaxy;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input, 999999);
  println!("{}", output);
}

fn part_two(input: &str, expansion_size: usize) -> u64 {
  let map = galaxy::parse_map(input);
  let galaxies = galaxy::find_galaxies(&map)
    .collect::<Vec<_>>();

  let new_galaxies = galaxy::expand_universe(&galaxies, (map.len(), map[0].len()), expansion_size);

  new_galaxies.iter()
    .enumerate()
    .map(|(idx, &galaxy_a)| {
      new_galaxies
      .iter()
        .skip(idx + 1)
        .map(|&galaxy_b| {
          let (row_a, col_a) = galaxy_a;
          let (row_b, col_b) = galaxy_b;
          let dist = (row_a as u64).abs_diff(row_b as u64)
            + (col_a as u64).abs_diff(col_b as u64);
          dist as u64
        })
        .sum::<u64>()
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two_expand_1() {
    assert_eq!(part_two("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 1), 374);
  }

  #[test]
  fn test_part_two_expand_10() {
    assert_eq!(part_two("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 9), 1030);
  }

  #[test]
  fn test_part_two_expand_100() {
    assert_eq!(part_two("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 99), 8410);
  }
}
