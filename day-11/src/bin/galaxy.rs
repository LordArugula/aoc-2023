use std::collections::HashMap;

pub fn parse_map(input: &str) -> Vec<Vec<char>> {
  let map = input
    .lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  map
}

pub fn find_galaxies<'a>(map: &'a Vec<Vec<char>>) -> impl Iterator<Item = (usize, usize)> + 'a {
  map.iter()
    .enumerate()
    .flat_map(|(row, l)| {
      l.iter()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(move |(col, _)| (row, col))
    })
}

pub fn expand_universe(
  galaxies: &Vec<(usize, usize)>,
  size: (usize, usize),
  expansion_size: usize
) -> Vec<(usize, usize)> {
  let (row_lookup, col_lookup) = galaxies.iter()
    .fold(
      (HashMap::new(), HashMap::new()),
      |(mut row_lookup, mut col_lookup), &(row, col)| {
        let row_vec = row_lookup.entry(row).or_insert(Vec::new());
        row_vec.push(col);
        let col_vec = col_lookup.entry(col).or_insert(Vec::new());
        col_vec.push(row);
        (row_lookup, col_lookup)
      },
    );

  let (map_height, map_width) = size;
  
  let empty_rows = (0..map_height)
    .filter(|row| !row_lookup.contains_key(row))
    .collect::<Vec<_>>();

  let empty_cols = (0..map_width)
    .filter(|col| !col_lookup.contains_key(col))
    .collect::<Vec<_>>();

  galaxies.iter()
    .map(move |&(row, col)| {
      let num_empty_rows = empty_rows.iter()
        .filter(|&&empty_row| empty_row < row)
        .count();

      let num_empty_cols = empty_cols.iter()
        .filter(|&&empty_col| empty_col < col)
        .count();
      (row + (num_empty_rows * expansion_size), col + (num_empty_cols * expansion_size))
    })
    .collect::<Vec<_>>()
}