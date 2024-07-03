fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u64 {
  let mut lines = input.lines();
  let instructions = lines.next()
    .expect("First line should be instructions");
  let num_instructions = instructions.len() as u64;
  let mut instructions = instructions.chars()
    .cycle();

  let nodes = lines.skip(1)
    .map(|l| {
      // line comes in format: XXX = (YYY, ZZZ)
      let node = &l[0..3];
      let left = &l[7..10];
      let right = &l[12..15];
      (node, (left, right))
    })
    .collect::<Vec<_>>();
  let node_map = nodes.iter()
    .map(|&(node, (left, right))| (node, (left, right)))
    .collect::<std::collections::HashMap::<_,_>>();

  let current_nodes = nodes.iter()
    .filter(|&&(node, _)| {
      node.ends_with('A')
    })
    .map(|&(node, _)| node)
    .collect::<Vec<_>>();
    
  let num_steps_per_path = current_nodes.iter()
    .map(|&node| {
      let mut num_steps = 0u64;
      let mut current = node;
      while !current.ends_with('Z') {
        num_steps += 1;
        let instruction = instructions.next()
          .expect("Should be an 'L' or 'R' character"); 
        let &(left, right) = node_map.get(current)
          .expect("Should have a node");
        current = match instruction {
          'L' => left,
          'R' => right,
          _ => panic!("Should be an 'L' or 'R' character")
        }
      }
      num_steps
    });

  num_steps_per_path.map(|num_steps| {
    match num_steps % num_instructions == 0 {
      true => num_steps / num_instructions,
      false => num_steps
    }
  }).fold(num_instructions, |acc, num_steps| {
    acc * num_steps
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"), 6);
  }
}
