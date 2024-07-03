fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 { 
  let mut input = input.split("\n\n");
  let seeds = input.next()
    .expect("First line should be seeds")
    ["seeds: ".len()..]
    .split_ascii_whitespace()
    .map(|seed| {
      seed.parse::<u32>()
        .expect("Should be a number")
    });

  let mappers = input
    .map(|mapper| {
      mapper.lines()
        .skip(1)
        .map(|l| {
          let mut range = l.split_ascii_whitespace();
            let destination_range = range.next()
              .expect("Should be destination range")
              .parse::<u32>()
              .expect("Should be a number");
            let source_range = range.next()
              .expect("Should be source range")
              .parse::<u32>()
              .expect("Should be a number");
            let range_length = range.next()
              .expect("Should be range length")
              .parse::<u32>()
              .expect("Should be a number");
            (destination_range, source_range, range_length)
          })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  seeds
    .map(|seed| {
      mappers.iter()
        .fold(seed, |value, mapper| {
          mapper.iter()
            .filter(|&&(_, source, length)| {
              value >= source && value - source < length
            })
            .next()
            .map_or(value, |&(destination, source, _)| {
              destination + (value - source)
            })
        })
    })
    .min()
    .expect("Should have lowest location number")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"), 35);
  }
}
