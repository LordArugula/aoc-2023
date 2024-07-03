fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
  Left,
  Up,
  Right,
  Down
}

struct CityBlock {
  row: usize,
  col: usize,
  heat_loss: u32
}

impl PartialEq for CityBlock {
  fn eq(&self, other: &Self) -> bool {
      other.heat_loss.eq(&self.heat_loss)
  }
}

impl PartialOrd for CityBlock {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      other.heat_loss.partial_cmp(&self.heat_loss)
  }
}

impl Eq for CityBlock {
}

impl Ord for CityBlock {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      other.heat_loss.cmp(&self.heat_loss)
  }
}

fn get_neighbors((row, col): (usize, usize), map: &Vec<Vec<u32>>) -> [Option<(usize, usize)>; 4] {
  let down = match row > 0 {
    true => Some((row - 1, col)),
    false => None,
  };

  let up = match row < map.len() - 1 {
    true => Some((row + 1, col)),
    false => None,
  };
  
  let left = match col > 0 {
    true => Some((row, col - 1)),
    false => None,
  };

  let right = match col < map[row].len() - 1 {
    true => Some((row, col + 1)),
    false => None,
  };

  [right, down, up, left]
}

fn part_one(input: &str) -> u32 {
  let map = input.lines()
    .map(|line| {
      line.chars()
         .map(|c| c.to_digit(10).expect("Should be a digit."))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let height = map.len();
  let width = map[height - 1].len();
  let end = (height - 1, width - 1);

  let mut open_space = std::collections::BinaryHeap::<CityBlock>::new();
  let mut visited = std::collections::HashMap::<(usize, usize), (usize, usize)>::new();
  let mut heat_losses = std::collections::HashMap::<(usize, usize), u32>::new();
  
  open_space.push(CityBlock { row: 0, col: 0, heat_loss: 0 });
  heat_losses.insert((0, 0), 0);
  
  while let Some(CityBlock { row, col, heat_loss: _ }) = open_space.pop() {
    if (row, col) == end {
      break;
    }

    let mut curr = (row, col);
    let mut direction = None;
    let mut num_in_same_direction = 0;
    while let Some(prev) = visited.get(&curr) {
      let curr_direction = match prev.0 == curr.0 {
          true if prev.1 > curr.1 => Direction::Left,
          true if prev.1 < curr.1 => Direction::Right,
          false if prev.0 > curr.0 => Direction::Down,
          false if prev.0 < curr.0 => Direction::Up,
          _ => panic!("Should not be at the same pos")
      };
      if let Some(dir) = direction {   
        if curr_direction != dir {
          break;
        }
      } else {
        direction = Some(curr_direction);
      }
      
      num_in_same_direction += 1;
      curr = *prev;
      
      if num_in_same_direction == 3 {
        break;
      }
    }
    
    get_neighbors((row, col), &map)
      .iter()
      .filter_map(|&neighbor| neighbor)
      .filter(|&(n_row, n_col)| {
        if let Some(dir) = direction {
          // cannot turn around
          // must change direction after three blocks
          match dir {
            Direction::Left if col < n_col => false,
            Direction::Left if col > n_col && num_in_same_direction == 3 => false,
            Direction::Right if col > n_col => false,
            Direction::Right if col < n_col && num_in_same_direction == 3 => false,
            Direction::Up if row > n_row => false, 
            Direction::Up if row < n_row && num_in_same_direction == 3 => false,
            Direction::Down if row < n_row => false,
            Direction::Down if row > n_row && num_in_same_direction == 3 => false,
            _ => true
          }
        } else {
          true 
        }
      })
      .for_each(|neighbor| {
        let next_heat_loss = heat_losses[&(row, col)] + map[neighbor.0][neighbor.1];
        if !heat_losses.contains_key(&neighbor) || next_heat_loss <= heat_losses[&neighbor] {
          heat_losses.insert(neighbor, next_heat_loss);
        println!("{:?} -> {:?} -> {:?} [{}]", visited.get(&(row, col)).unwrap_or(&(0, 0)), (row, col), neighbor, next_heat_loss);
          open_space.push(CityBlock { 
            row: neighbor.0, 
            col: neighbor.1, 
            heat_loss: next_heat_loss
          });
          visited.insert(neighbor, (row, col));
        }
      });
  }

  let mut curr = end;
  println!("{:?}", curr);
  while let Some(&(row, col)) = visited.get(&curr) {
    curr = (row, col);
    println!("{:?}", curr);
  }
  
  heat_losses[&end]
}

/*
2413432311323    2>>34^>>>1323
3215453535623    32v>>>35v5623
3255245654254    32552456v>>54
3446585845452    3446585845v52
4546657867536    4546657867v>6
1438598798454    14385987984v4
4457876987766    44578769877v6
3637877979653    36378779796v>
4654967986887    465496798688v
4564679986453    456467998645v
1224686865563    12246868655<v
2546548887735    25465488877v5
4322674655533    43226746555v>
*/

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"), 102);
  }
}
