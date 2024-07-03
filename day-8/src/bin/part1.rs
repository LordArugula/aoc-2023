fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  let mut lines = input.lines();
  let mut instructions = lines.next()
    .expect("First line should be instructions")
    .chars()
    .cycle();

  let node_map = lines.skip(1)
    .map(|l| {
      // line comes in format: XXX = (YYY, ZZZ)
      let node = &l[0..3];
      let left = &l[7..10];
      let right = &l[12..15];
      (node, (left, right))
    })
    .collect::<std::collections::HashMap<_, _>>();
  let mut current_node = "AAA";

  let mut num_steps = 0;
  while current_node != "ZZZ" {
    num_steps += 1;
    let instruction = instructions.next()
      .expect("Should be an 'L' or 'R' character");
    let &(left, right) = node_map.get(&current_node)
      .expect("Should exist");
    current_node = match instruction {
      'L' => left,
      'R' => right,
      _ => panic!("Should be an 'L' or 'R' character"),
    };
  }
  
  num_steps
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"), 2);
    
    assert_eq!(part_one("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"), 6);
  }
}
