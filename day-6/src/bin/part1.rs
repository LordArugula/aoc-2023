fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let mut lines = input.lines();
  let times = lines.next().expect("First line should be times");
  let distances = lines.next().expect("Second line should be distances");
  times.split_ascii_whitespace()
    .zip(distances.split_ascii_whitespace())
    .skip(1)
    .map(|(time, distance)| {
      let time = time.parse::<u32>().expect("Should be a number");
      let dist = distance.parse::<u32>().expect("Should be a number");
      (time, dist)
    })
    .map(|(race_duration, distance_to_beat)| {
      let mut left = 0;
      let mut right = race_duration;
      let mut current = (right + left) / 2;
      let mut last_winning_time = current;
      while right - left > 1 {
        let dist = (race_duration - current) * current;
        if dist <= distance_to_beat {
          left = current;
          current = (right + left) / 2;
        } else {
          last_winning_time = current;
          right = current;
          current = (right + left) / 2;
        }
      }
      race_duration - last_winning_time * 2 + 1
    })
    .fold(1, |acc, val| acc * val)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("Time:      7  15   30
Distance:  9  40  200"), 288);
  }
}
