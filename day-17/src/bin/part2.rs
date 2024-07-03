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
    assert_eq!(part_two("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"), 0);
  }
}
