mod beams;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let grid = input.lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  beams::calculate_energized_tiles(&grid, beams::Direction::Right, 0, 0)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#), 46);
  }
}
