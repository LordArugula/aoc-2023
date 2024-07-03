fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u32 {

  let mut input = input.split("\n\n");
  let binding = input.next()
    .expect("First line should be seeds")
    ["seeds: ".len()..]
    .split_ascii_whitespace()
    .collect::<Vec<_>>();
  let seeds = binding
    .chunks(2)
    .map(|range| {
      let start = range[0].parse::<u32>().expect("Should be a number");
      let length = range[1].parse::<u32>().expect("Should be a number");
      (start,length)
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

  let result = seeds.map(|seed_range| {
    mappers.iter()
      .fold(vec![seed_range], |input_ranges, mapper| {
        // println!("Starting with {:?}", input_ranges);
        let ranges = input_ranges.iter()
          .flat_map(|&(input_start, input_length)| {
            let ranges = mapper.iter()
              .filter_map(move |&(_, source, length)| {
                // println!("Splitting {:?} by {:?}", (input_start, input_length), (source, length));
                if is_inside((source, length), (input_start, input_length)) {
                  // input range completely overlaps map range
                  // println!("beyond");
                  let (a, overlap) = split_range(input_start, input_length, source);
                  let (overlap, b) = split_range(overlap.0, overlap.1, source + length);
                  Some(vec![a, overlap, b])
                } else if overlaps_left((input_start, input_length), (source, length)) {
                  // input range overlaps from the left
                  // println!("left");
                  let (a, overlap) = split_range(input_start, input_length, source);
                  Some(vec![a, overlap])
                } else if overlaps_right((input_start, input_length), (source, length)) {
                  // input range overlaps from the right
                  // println!("right");
                  let (overlap, _) = split_range(input_start, input_length, source + length);
                  Some(vec![overlap])
                } else if is_inside((input_start, input_length), (source, length)) {
                  // input range is inside map range
                  // println!("inside");
                  Some(vec![(input_start, input_length)])
                } else {
                  // input range has no overlap and is outside map range
                  // println!("outside");
                  None::<Vec<_>>
                }
              })
              .flatten()
              .collect::<Vec<_>>();
            let result = {
                if ranges.is_empty() {
                  vec![(input_start, input_length)]
                } else {
                  ranges.iter()
                    .map(|&(input_start, input_length)| {
                      mapper.iter()
                        .filter(|&&(_, source, length)| {
                          is_inside((input_start, input_length), (source, length))
                        })
                        .next()
                        .map_or((input_start, input_length), |&(destination, source, _)| {
                          (destination + (input_start - source), input_length)
                        })
                    })
                  .collect::<Vec<_>>()
                }
              };
            result
          })
          .collect::<Vec<_>>();
        // println!("Ending with {:?}", ranges);
        ranges
      })
  })
    .flatten()
    .map(|(start, _)| {
      start
    })
    .filter(|&x| x > 0)
    .min()
    .expect("Should have lowest location");
  result
}

/// Tests if a is inside b
fn is_inside(a: (u32, u32), b: (u32, u32)) -> bool {
  let (a_start, a_length) = a;
  let (b_start, b_length) = b;
  // a starts after b starts
  // a ends before b ends
  if a_start >= b_start {
    // avoid overflow
    // && a_start + a_length <= b_start + b_length
    if a_length < b_length {
      return a_start - b_start <= b_length - a_length;
    } else {
      return (a_start - b_start) + (a_length - b_length) <= 0; 
    }
  }
  false
}

fn overlaps_left(a: (u32, u32), b: (u32, u32)) -> bool {
  let (a_start, a_length) = a;
  let (b_start, b_length) = b;
  // a starts before b starts
  // a ends after b starts
  // a ends before b ends
  if a_start <= b_start 
    // avoid overflow
    // && a_start + a_length > b_start
    && a_length > b_start - a_start {
    // avoid overflow
    // && a_start + a_length < b_start + b_length
    if a_length > b_length {
      return a_length - b_length < b_start - a_start;
    } else {
      return 0 < (b_start - a_start) + (b_length - a_length);
    }
  }
  false
}

fn overlaps_right(a: (u32, u32), b: (u32, u32)) -> bool {
  let (a_start, a_length) = a;
  let (b_start, b_length) = b;
  // a starts after b starts
  // a starts before b ends
  // a ends after b ends
  if a_start > b_start 
    // avoid overflow
    // && a_start < b_start + b_length
    && a_start - b_start <= b_length {
      // avoid overflow
      // && a_start + a_length > b_start + b_length
      // && a_start - b_start + a_length > b_length
      if a_length > b_length {
        return (a_start - b_start) + (a_length - b_length) > 0;
      } else {
        return a_start - b_start > b_length - a_length;
      }
  }
  false
}

fn split_range(start: u32, length: u32, split: u32) -> ((u32, u32), (u32, u32)) {
  ((start, split - start), (split, length - (split - start)))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
      assert_eq!(part_two("seeds: 79 14 55 13

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
  56 93 4"), 46);
  }
}
