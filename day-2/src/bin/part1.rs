fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input, 12, 13, 14);
  println!("{}", output);
}

fn part_one(input: &str, red: u32, green: u32, blue: u32) -> u32 {
  return input
    .lines()
    .map(|l| {
      let after_game_id = l
        .find(": ")
        .expect("Should have a colon ':' followed by a space");

      let game_len = "Game ".len();
      let game_id: u32 = l[game_len..after_game_id]
        .parse()
        .expect("Should be a number");

      let is_game_valid = l[(after_game_id + 2)..]
        .split_terminator("; ")
        .map(|set| set.split_terminator(", "))
        .map(|set| {
          set.map(|cubes| {
            let (number, color) = cubes
              .split_once(' ')
              .expect("Should be a number and color separated by a space");
            (number.parse::<u32>().expect("Should be a number"), color)
          })
        })
        .all(|mut set| {
          set.all(|(num, color)| match color {
            "red" => num <= red,
            "blue" => num <= blue,
            "green" => num <= green,
            _ => false,
          })
        });

      if is_game_valid {
        return game_id;
      }
      0
    })
    .sum();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(
      part_one(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        12,
        13,
        14
      ),
      8
    );
  }
}
