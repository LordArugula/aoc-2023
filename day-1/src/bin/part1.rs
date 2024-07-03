fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let result : u32 = input
    .lines()
    .map(|l| {
      let mut digits = l.chars()
        .filter_map(|c| c.to_digit(10));
      let first = digits.next().expect("Should have at least one digit");
      let last = digits.last().unwrap_or(first);
      first * 10u32 + last
    })
    .sum();

  return result;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
     assert_eq!(part_one("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142); 
  }
}