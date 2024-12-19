use std::{collections::HashSet, ops::Add};

use aoc::bootstrap;

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<u8>> {
  lines
    .iter()
    .map(|l| {
      l.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap_or(u8::MAX))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn get_surroundings(pos: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
  [
    (pos.0, pos.1.saturating_sub(1)),
    (pos.0.saturating_add(1), pos.1),
    (pos.0, pos.1.saturating_add(1)),
    (pos.0.saturating_sub(1), pos.1),
  ]
  .iter()
  .filter(|&next| *next != pos && next.0 < bounds.0 && next.1 < bounds.1)
  .map(|next| *next)
  .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct WalkResult {
  peaks: HashSet<(usize, usize)>,
  ratings: usize,
}

impl WalkResult {
  fn new_peak(peak_pos: (usize, usize)) -> Self {
    let peaks = HashSet::from([peak_pos]);
    let ratings = 1;
    Self { peaks, ratings }
  }
}

impl Default for WalkResult {
  fn default() -> Self {
    let peaks = HashSet::new();
    let ratings = 0;
    Self { peaks, ratings }
  }
}

impl Add for WalkResult {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let peaks = self.peaks.union(&rhs.peaks).map(|v| *v).collect();
    let ratings = self.ratings + rhs.ratings;
    Self { peaks, ratings }
  }
}

fn walk(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> WalkResult {
  let bounds = (grid[0].len(), grid.len());
  let value = grid[pos.1][pos.0];

  if value == 9 {
    return WalkResult::new_peak(pos);
  }

  get_surroundings(pos, bounds)
    .iter()
    .filter(|&s| grid[s.1][s.0] == value + 1)
    .map(|&s| walk(grid, s))
    .fold(WalkResult::default(), |acc, res| acc + res)
}

fn find_positions(grid: &Vec<Vec<u8>>, value: u8) -> Vec<(usize, usize)> {
  let bounds = (grid[0].len(), grid.len());
  let mut result: Vec<(usize, usize)> = vec![];

  for y in 0..bounds.1 {
    for x in 0..bounds.0 {
      if grid[y][x] == value {
        result.push((x, y));
      }
    }
  }

  result
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let walk_results = find_positions(&input, 0)
    .iter()
    .map(|start| walk(&input, *start))
    .collect::<Vec<_>>();

  let part_1 = walk_results
    .iter()
    .map(|res| res.peaks.len())
    .sum::<usize>();
  output.write_part(1, &part_1);

  let part_2 = walk_results.iter().map(|res| res.ratings).sum::<usize>();
  output.write_part(2, &part_2);
}
