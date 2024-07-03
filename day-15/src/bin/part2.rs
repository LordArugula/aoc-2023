fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn hash_label(label: &str) -> u32 {
  label.chars().fold(0, |acc, c| ((acc + u32::from(c)) * 17) % 256)
}

fn dash_operation(label: &str, boxes: &mut std::collections::HashMap<u32, Vec<(&str, u32)>>) {
  let label_hash = hash_label(label);
  if let Some(fbox) = boxes.get_mut(&label_hash) {
    if let Some(pos) = fbox.iter().position(|&(box_label, _)| box_label == label) {
      fbox.remove(pos);
    }
  }
}

fn equal_operation<'a>(label: &'a str, focal_length: u32, boxes: &mut std::collections::HashMap<u32, Vec<(&'a str, u32)>>) {
  let label_hash = hash_label(label);
  boxes.entry(label_hash)
    .and_modify(|fbox| {
      if let Some(pos) = fbox.iter().position(|&(box_label, _)| box_label == label) {
        fbox[pos] = (label, focal_length);
      } else {
        fbox.push((label, focal_length));
      }
    })
    .or_insert(vec![(label, focal_length)]);
}

fn part_two(input: &str) -> u32 {
  let boxes = input.split(',')
    .fold(std::collections::HashMap::new(), |mut boxes, step| {
      match step.ends_with('-') {
        true => {
          let label = &step[..step.len() - 1];
          dash_operation(label, &mut boxes);
        },
        false => {
          let (label, focal_length) = step
            .rsplit_once('=')
            .expect("Should have an '='");
          
          let focal_length = focal_length.parse::<u32>()
            .expect("Should be a number");
          equal_operation(label, focal_length, &mut boxes);
        },
      }
      boxes
    });
  
  
  (0..256)
    .map(|box_idx| (box_idx, boxes.get(&box_idx)))
    .filter(|(_, fbox)| fbox.is_some())
    .map(|(box_idx, fbox)| {
      let fbox = fbox.unwrap();
      fbox.iter()
        .enumerate()
        .map(|(slot_idx, &(_, focal_length))| {
          (box_idx + 1) * (slot_idx + 1) as u32 * focal_length
        })
        .sum::<u32>()
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 145);
  }
}
