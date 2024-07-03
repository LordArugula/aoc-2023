fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 {
  let result: u32 = input
    .lines()
    .map(|l| {
      let mut first = None;
      let len = l.len();
      for (idx, c) in l.char_indices() {
        if c.is_digit(10) {
          first = c.to_digit(10);
          break;
        }
        let x = &l[idx..(idx+3 < len).then(|| idx+3).unwrap_or(len)];
        match x {
          "one" => {
            first = Some(1);
            break;
          }
          "two" => {
            first = Some(2);
            break;
          }
          "six" => {
            first = Some(6);
            break;
          }
          _ => {}
        };

        let y = &l[idx..(idx+4 < len).then(|| idx+4).unwrap_or(len)];
        match y {
          "four" => {
            first = Some(4);
            break;
          }
          "five" => {
            first = Some(5);
            break;
          }
          "nine" => {
            first = Some(9);
            break;
          }
          _ => {}
        };

        let z = &l[idx..(idx+5 < len).then(|| idx+5).unwrap_or(len)];
        match z {
          "three" => {
            first = Some(3);
            break;
          }
          "seven" => {
            first = Some(7);
            break;
          }
          "eight" => {
            first = Some(8);
            break;
          }
          _ => {}
        };
      }

      let mut last = None;
      for (idx, c) in l.char_indices().rev() {
        if c.is_digit(10) {
          last = c.to_digit(10);
          break;
        }
        let idx = idx + 1;
        let x = &l[((idx >= 3).then(|| idx - 3).unwrap_or(0))..idx];
        match x {
          "one" => {
            last = Some(1);
            break;
          }
          "two" => {
            last = Some(2);
            break;
          }
          "six" => {
            last = Some(6);
            break;
          }
          _ => {}
        };

        let y = &l[((idx >= 4).then(|| idx - 4).unwrap_or(0))..idx];
        match y {
          "four" => {
            last = Some(4);
            break;
          }
          "five" => {
            last = Some(5);
            break;
          }
          "nine" => {
            last = Some(9);
            break;
          }
          _ => {}
        };

        let z = &l[(idx >= 5).then(|| idx - 5).unwrap_or(0)..idx];
        match z {
          "three" => {
            last = Some(3);
            break;
          }
          "seven" => {
            last = Some(7);
            break;
          }
          "eight" => {
            last = Some(8);
            break;
          }
          _ => {}
        };
      }

      return first.expect("Should have a number") * 10 + last.expect("Should have a number");
    })
    .sum();

  return result;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(
      part_two(
          "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
      ),
      281
    );
  }
}
