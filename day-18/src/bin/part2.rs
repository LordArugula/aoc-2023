fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u64 {
  let mut points = input.lines()
    .map(|line| {
      let start_of_hex = line.find('#').expect("Should have '#'") + 1;
      let distance = &line[(start_of_hex )..(start_of_hex + 5)];
      let direction = &line[(start_of_hex + 5)..(start_of_hex + 6)];
      let distance = i64::from_str_radix(distance, 16)
        .expect("Should be valid hexadecimal number");
      return (direction, distance);   
    })
    .fold(
      Vec::new(), 
      |mut nodes, (direction, distance)| {
        let &(x, y) = nodes.last().unwrap_or(&(0, 0));
        let position = match direction {
          "0" => (x + distance, y),
          "1" => (x, y - distance),
          "2" => (x - distance, y),
          "3" => (x, y + distance),
          _ => panic!("Should only be 'L', 'R', 'U', or 'D'")
        };
      nodes.push(position);
      nodes
    });

  points.push(points[0]);
  let perimeter = points.windows(2)
    .map(|w| {
      let (x1, y1) = w[0];
      let (x2, y2) = w[1];
      x1.abs_diff(x2) + y1.abs_diff(y2)
    })
    .sum::<u64>() / 2 + 1;

  let area = points.windows(2)
    .map(|w| {
      let (x1, y1) = w[0];
      let (x2, y2) = w[1];
      (x1 * y2) - (x2 * y1)
    })
    .sum::<i64>()
    .abs() as u64 / 2;

  println!("Points:{:?}\nArea: {}, Perimeter: {}", points, area, perimeter);
  area + perimeter
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"), 952408144115);
  }
}
