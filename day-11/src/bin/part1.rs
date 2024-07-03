mod galaxy;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let expansion_size = 1;
  let map = galaxy::parse_map(input);
  let galaxies = galaxy::find_galaxies(&map)
    .collect::<Vec<_>>();

  let new_galaxies = galaxy::expand_universe(&galaxies, (map.len(), map[0].len()), expansion_size);

  new_galaxies.iter()
    .enumerate()
    .map(|(idx, &galaxy_a)| {
      new_galaxies
      .iter()
        .skip(idx)
        .map(|&galaxy_b| {
          let (row_a, col_a) = galaxy_a;
          let (row_b, col_b) = galaxy_b;
          let dist = row_a.abs_diff(row_b) + col_a.abs_diff(col_b);
          dist as u32
        })
        .sum::<u32>()
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 374);
  }
}
