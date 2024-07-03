fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u32 {
  
  let mut points = input.lines()
    .map(|line| {
      let mut split = line.split_ascii_whitespace();
      let direction = split.next().expect("Should be direction");
      let distance = split.next()
        .expect("Should be distance")
        .parse::<i32>()
        .expect("Should be a number");
      let color = split.next()
        .expect("Should be color");
      (direction, distance, color)
    })
    .fold(Vec::new(), |mut nodes, (direction, distance, _)| {
      let &(x, y) = nodes.last().unwrap_or(&(0, 0));
      let position = match direction {
        "D" => (x, y - distance),
        "U" => (x, y + distance),
        "R" => (x + distance, y),
        "L" => (x - distance, y),
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
    .sum::<u32>() / 2 + 1;

  let area = points.windows(2)
    .map(|w| {
      let (x1, y1) = w[0];
      let (x2, y2) = w[1];
      (x1 * y2) - (x2 * y1)
    })
    .sum::<i32>()
    .abs() as u32 / 2;

  println!("Points:{:?}\nArea: {}, Perimeter: {}", points, area, perimeter);
  area + perimeter
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("R 6 (#70c710)
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
U 2 (#7a21e3)"), 62);
  }
}
