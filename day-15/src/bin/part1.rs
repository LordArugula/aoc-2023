fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  input.split(',')
    .map(|step| {
      step.chars()
        .fold(0, |acc, c| {
          ((acc + u32::from(c)) * 17) % 256
        })
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 1320);
  }
}
