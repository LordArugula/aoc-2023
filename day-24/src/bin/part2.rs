fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(""), 0);
  }
}
