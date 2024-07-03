mod path;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let map = path::parse_map(input);
  let path = path::find_path(&map);

  println!("{:?}", path);
  path.len() as u32 / 2
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("-L|F7
7S-7|
L|7||
-L-J|
L|-JF"), 4);
  }

  #[test]
  fn test_part_one_8() {
    assert_eq!(part_one("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"), 8);
  }
}
