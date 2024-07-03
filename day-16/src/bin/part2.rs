mod beams;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 {
  let grid = input.lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  
  (0..grid.len())
    .map(|row| {
      (beams::Direction::Right, (row, 0))
    })
    .chain((0..grid.len()).map(|row| (beams::Direction::Left, (row, grid[row].len() - 1))))
    .chain((0..grid[0].len()).map(|col| (beams::Direction::Down, (grid.len() - 1, col))))
    .chain((0..grid[0].len()).map(|col| (beams::Direction::Up, (0, col))))
    .map(|(direction, (row, col))| {
      beams::calculate_energized_tiles(&grid, direction, row, col)
    })
    .max()
    .expect("Should have the max number of energized tiles")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#), 51);
  }
}
