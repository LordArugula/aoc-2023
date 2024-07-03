fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  input.lines()
    .map(|l| {
      let end_of_springs = l.find(' ')
        .expect("Springs and groups are separated by a space");

      let springs = l[0..end_of_springs]
        .chars()
        .collect::<Vec<_>>();
      let groups = l[end_of_springs + 1..]
        .split(',')
        .map(|c| c.parse::<usize>().expect("Should be a number"))
        .collect::<Vec<_>>();
      (springs, groups)
    })
    .map(|(springs, group_lengths)| {
      let mut spr_idx = 0;
      let mut grp_idx = 0;
      while spr_idx < springs.len() {
        let start = spr_idx;
        match springs[spr_idx] {
          '?' | '#' => {
            // read until we reach a '.'
            while spr_idx < springs.len() {
              if springs[spr_idx] == '.' {
                break;
              }
              spr_idx += 1;
            }
            let group = &springs[start..spr_idx];
            match group.len() < group_lengths[grp_idx] {
              true => {
                // this group is too small
              }
              false => {
                // 
              }
            }
          },
          _ => {}
        }
        spr_idx += 1;
      }
      0
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"), 21);
  }
}
