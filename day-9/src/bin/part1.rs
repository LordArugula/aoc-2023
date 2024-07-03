fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> i32 { 
  input.lines()
    .map(|l| {
      l.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().expect("Should be a number"))
        .collect::<Vec<_>>()
    })
    .map(|history| {
      let mut ends = Vec::<i32>::new();
      ends.push(*history.last().expect("Should have something here"));

      let mut curr_history = history;
      while curr_history.iter().any(|x| *x != 0) {
        curr_history = curr_history.windows(2)
          .map(|pair| pair[1] - pair[0])
          .collect::<Vec<_>>();
  
        match curr_history.last() {
          Some(x) => ends.push(*x),
          None => {}
        };
      }

      let result = ends.iter()
        .rev()
        .fold(0, |acc, x| {
          acc + x
        });
      result
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"), 114);
  }
}
