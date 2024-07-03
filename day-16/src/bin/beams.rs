#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Direction {
  Left,
  Up,
  Right,
  Down,
  Horizontal,
  Vertical
}

pub fn get_direction(symbol: char, direction: Direction) -> Direction {
  match symbol {
    '/' if matches!(direction, Direction::Left) => Direction::Up,
    '/' if matches!(direction, Direction::Up) => Direction::Left,
    '/' if matches!(direction, Direction::Right) => Direction::Down,
    '/' if matches!(direction, Direction::Down) => Direction::Right,
    '\\' if matches!(direction, Direction::Left) => Direction::Down,
    '\\' if matches!(direction, Direction::Up) => Direction::Right,
    '\\' if matches!(direction, Direction::Right) => Direction::Up,
    '\\' if matches!(direction, Direction::Down) => Direction::Left,
    '|' if matches!(direction, Direction::Left | Direction::Right) => Direction::Vertical,
    '-' if matches!(direction, Direction::Up | Direction::Down) => Direction::Horizontal,
    _ => direction,
  }
}

pub fn calculate_energized_tiles(grid: &Vec<Vec<char>>, direction: Direction, row: usize, col: usize) -> u32 {
  let mut beams = vec![(get_direction(grid[row][col], direction), (row, col))];
  let mut visited = std::collections::HashSet::<(Direction, (usize, usize))>::new();

  while let Some(beam) = beams.pop() {
    let (mut direction, (mut row, mut col)) = beam;

    // move in direction until we hit the edge`
    loop {
      if visited.contains(&(direction.clone(), (row, col))) {
        break;
      }

      visited.insert((direction.clone(), (row, col)));

      let next = match direction {
        Direction::Left if col > 0 => Some((row, col - 1)),
        Direction::Up if row < grid.len() - 1 => Some((row + 1, col)),
        Direction::Right if col < grid[row].len() - 1 => Some((row, col + 1)),
        Direction::Down if row > 0 => Some((row - 1, col)),
        _ => None
      };

      if let Some((next_row, next_col)) = next {
        direction = get_direction(grid[next_row][next_col], direction);
        direction = match direction {
          Direction::Horizontal => {
            beams.push((Direction::Right, (next_row, next_col)));
            Direction::Left
          },
          Direction::Vertical => {
            beams.push((Direction::Down, (next_row, next_col)));
            Direction::Up
          },
          _ => direction
        };

        (row, col) = (next_row, next_col);
      }
      else {
        break; 
      }
    }
  }

  visited.iter()
    .map(|(_, position)| position)
    .collect::<std::collections::HashSet<_>>()
    .len() as u32
}