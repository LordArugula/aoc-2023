fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 { 
  let num_winning_numbers_per_card = input.lines()
    .map(|l| {
      let end_of_card_number = l.find(':').expect("Should have a ':' after the card number");
      &l[(end_of_card_number + 1)..]
    })
    .map(|l| l.split_once('|').expect("Should have a '|' separating numbers"))
    .map(|(winning_numbers, given_numbers)| {
      let winning_numbers = winning_numbers.split_ascii_whitespace()
        .map(|n| n.parse::<u32>().expect("Should be a number"));
      let given_numbers = given_numbers.split_ascii_whitespace()
        .map(|n| n.parse::<u32>().expect("Should be a number"));
      (winning_numbers, given_numbers)
    })
    .map(|(winning_numbers, given_numbers)| {
      let winning_numbers = winning_numbers.collect::<std::collections::HashSet<u32>>();
      let num_winning_numbers_per_card = given_numbers.filter(move |n| winning_numbers.contains(n))
        .count();
      num_winning_numbers_per_card as u32
    })
    .collect::<Vec<_>>();
 
  let mut num_cards = (0..num_winning_numbers_per_card.len())
    .map(|_| 1)
    .collect::<Vec<_>>();
  num_winning_numbers_per_card.iter()
    .enumerate()
    .for_each(|(idx, num)| {
      for card_id in ((idx + 1)..=(idx + *num as usize)) {
        num_cards[card_id] += num_cards[idx];
      }
    });
  num_cards.iter().sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
      assert_eq!(part_two("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
  }
}
