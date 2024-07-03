fn get_connected_neighbors(pos: (usize, usize), map: &Vec<Vec<char>>) -> [Option<(usize, usize)>; 4] {
  let (row, col) = pos;
  let pipe = map[row][col];

  let up = match pipe {
    '|' | 'L' | 'J' | 'S' if row > 0 => {
      if let '|' | 'F' | '7' | 'S' = map[row - 1][col] {
        Some((row - 1, col))
      } else { 
        None 
      }
    },
    _ => None
  };

  let down = match pipe {
    '|' | 'F' | '7' | 'S' if row < map.len() - 1 => {
      if let '|' | 'L' | 'J' | 'S' = map[row + 1][col] {
        Some((row + 1, col))
      } else {
        None
      }
    },
    _ => None
  };

  let left = match pipe {
    '-' | 'J' | '7' | 'S' if col > 0 => {
      if let '-' | 'F' | 'L' | 'S' = map[row][col - 1] {
        Some((row, col - 1))
      } else {
        None
      }
    },
    _ => None
  };

  let right = match pipe {
    '-' | 'F' | 'L' | 'S' if col < map[row].len() - 1 => {
      if let '-' | 'J' | '7' | 'S' = map[row][col + 1] {
        Some((row, col + 1))
      } else {
        None
      }
    },
    _ => None
  };

  [up, right, down, left]
}

pub fn get_neighbors(pos: (usize, usize), map: &Vec<Vec<char>>) -> [Option<(usize, usize)>; 4] {
  let (row, col) = pos;
  
  let up = match row > 0 {
    true => Some((row - 1, col)),
    false => None,
  };
 
  let down = match row < map.len() - 1 {
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

  [up, right, down, left]
}

pub fn parse_map(input: &str) -> Vec<Vec<char>> {
  let map = input.lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  
  map
}

pub fn find_path(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let start = map.iter()
    .enumerate()
    .find_map(|(row, l)| {
      match l.iter().position(|&c| c == 'S') {
          Some(col) => Some((row, col)),
          None => None  
      }
    })
    .expect("Should have an 'S'");

  let mut path = Vec::<(usize, usize)>::new();
  path.push(start);

  let next = get_connected_neighbors(start, &map)
    .iter()
    .find_map(|&neighbor| neighbor)
    .expect("Should be connected to something");

  let mut previous = start;
  let mut current = next;
  loop {
    path.push(current);

    let next = get_connected_neighbors(current, &map)
      .iter()
      .filter_map(|&neighbor| neighbor)
      .filter(|&neighbor| {
        neighbor != previous
      })
      .next()
      .expect("Should have a next");

    previous = current;
    current = next;

    if current == start {
      break;
    }
  }
  
  return path;
}