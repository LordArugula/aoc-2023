fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

enum Hand {
  FiveOfAKind = 7,
  FourOfAKind = 6,
  FullHouse = 5,
  ThreeOfAKind = 4,
  TwoPair = 3,
  OnePair = 2,
  HighCard = 1
}

fn part_two(input: &str) -> u32 {  let mut hands = input.lines()
    .map(|line| {
      let separator_index = line.find(' ')
        .expect("Should have a space separating cards and bid");
      let hand = &line[..separator_index];
      let bid = line[separator_index+1..]
        .parse::<u32>()
        .expect("Should be a number");
      (hand, bid)
    })
    .map(|(hand, bid)| {
      let hand_tabled = hand.chars()
        .fold(std::collections::HashMap::with_capacity(5), |mut table, c| {
          table.entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
          table
        });
      let rank = match hand_tabled.len() {
        // hand is five of a kind
        1 => Ok(Hand::FiveOfAKind),
        // hand is either four of a kind, 
        // full house (three of a kind + two of a kind),
        2 => {
          let &num_jokers = hand_tabled.get(&'J').unwrap_or(&0);
          let mut card_counts = hand_tabled.values();
          let first = card_counts.next().expect("Should be first card");
          let second = card_counts.next().expect("Should be second card");
          match (first, second) {
            (4,1) | (1,4) => {
              match num_jokers {
                0 => Ok(Hand::FourOfAKind),
                1 | 4 => Ok(Hand::FiveOfAKind),
                _ => Err("Should have either one or four Jokers in this hand")
              }
            },
            (2,3) | (3,2) => {
              match num_jokers {
                0 => Ok(Hand::FullHouse),
                2 | 3 => Ok(Hand::FiveOfAKind),
                _ => Err("Should have either two or three Jokers in this hand")
              }
            },
            _ => Err("Should be either four of a kind or a full house")
          }
        },
        // hand is either three of a kind
        // or two pair (two of a kind + two of a kind)
        3 => {
          let &num_jokers = hand_tabled.get(&'J').unwrap_or(&0);
          let mut card_counts = hand_tabled.values();
          let first = card_counts.next().expect("Should be first card");
          let second = card_counts.next().expect("Should be second card");
          let third = card_counts.next().expect("Should be third card");
          match (first, second, third) {
            (3,1,1) | (1,3,1) | (1,1,3) => {
              match num_jokers {
                0 => Ok(Hand::ThreeOfAKind),
                1 => Ok(Hand::FourOfAKind),
                3 => Ok(Hand::FourOfAKind),
                _ => Err("Should have either one or three Jokers in this hand")
              }
            },
            (2,2,1) | (2,1,2) | (1,2,2) => {
              match num_jokers {
                0 => Ok(Hand::TwoPair),
                1 => Ok(Hand::FullHouse), 
                2 => Ok(Hand::FourOfAKind),
                _ => Err("Should have either one or two Jokers in this hand")
              }
            },
            _ => Err("Should be either three of a kind or a two pair")
          }
        },
        // hand is one pair
        4 => {
          let &num_jokers = hand_tabled.get(&'J').unwrap_or(&0);
          match num_jokers {
            0 => Ok(Hand::OnePair),
            1 => Ok(Hand::ThreeOfAKind),
            2 => Ok(Hand::ThreeOfAKind),
            _ => Err("Should have either one or two Jokers in this hand")
          }
        },
        // hand is high card (all different)
        5 => {
          let &num_jokers = hand_tabled.get(&'J').unwrap_or(&0);
          match num_jokers {
            0 => Ok(Hand::HighCard),
            1 => Ok(Hand::OnePair),
            _ => Err("Should have either one or no Jokers in this hand")
          }
        },
        _ => Err("Should have five cards in hand"),
      };
      (hand, bid, rank.expect("Should have a rank") as i32)
    })
    .collect::<Vec<_>>();

  hands.sort_by(|&hand_a, &hand_b| {
    let (hand_a, _, rank_a) = hand_a;
    let (hand_b, _, rank_b) = hand_b;
    if rank_a > rank_b {
      std::cmp::Ordering::Greater
    } else if rank_a == rank_b {
      let hand_a = hand_a.chars().map(|c| {
        match c {
          'A' => Ok(14),
          'K' => Ok(13),
          'Q' => Ok(12),
          'J' => Ok(1),
          'T' => Ok(10),
          '2'..='9' => Ok(c.to_digit(10).expect("Should be a number")),
          _ => Err(0)
        }
          .expect("Should be a character in AKQTJ23456789")
      });
      let hand_b = hand_b.chars().map(|c| {
        match c {
          'A' => Ok(14),
          'K' => Ok(13),
          'Q' => Ok(12),
          'J' => Ok(1),
          'T' => Ok(10),
          '2'..='9' => Ok(c.to_digit(10).expect("Should be a number")),
          _ => Err(0)
        }
          .expect("Should be a character in AKQTJ23456789")
      });
      let cmp_candidates = hand_a.zip(hand_b)
        .skip_while(|&(a, b)| a == b)
        .next();
      match cmp_candidates {
        Some((a,b)) => a.cmp(&b),
        _ => std::cmp::Ordering::Less
      }
    } else {
      std::cmp::Ordering::Less
    }
  });

  hands.iter()
    .enumerate()
    .fold(0, |acc, (idx, hand)| {
      acc + ((idx + 1) as u32) * hand.1
    })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 5905);
  }
}
