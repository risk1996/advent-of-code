use std::{collections::HashSet, ops::Add};

use aoc::bootstrap;

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<char>> {
  lines
    .iter()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Region {
  label: char,
  area: usize,
  circumference: usize,
}

impl Region {
  fn fence_cost(&self) -> usize {
    self.area * self.circumference
  }
}

impl Add for Region {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let label = self.label;
    let area = self.area + rhs.area;
    let circumference = self.circumference + rhs.circumference;

    Region {
      label,
      area,
      circumference,
    }
  }
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

fn visit(
  grid: &Vec<Vec<char>>,
  pos: (usize, usize),
  visited: &mut HashSet<(usize, usize)>,
) -> Region {
  visited.insert(pos);
  let bounds = (grid[0].len(), grid.len());
  let value = grid[pos.1][pos.0];
  let surroundings = get_surroundings(pos, bounds)
    .iter()
    .filter(|s| grid[s.1][s.0] == value)
    .map(|s| *s)
    .collect::<Vec<_>>();

  let mut result = Region {
    label: value,
    area: 1,
    circumference: 4 - surroundings.len(),
  };

  for s in surroundings {
    if !visited.contains(&s) {
      result = result + visit(&grid, s, visited);
    }
  }

  result
}

fn compute_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
  let bounds = (grid[0].len(), grid.len());
  let mut visited = HashSet::<(usize, usize)>::new();

  (0..bounds.1)
    .flat_map(|y| (0..bounds.0).map(move |x| (x, y)))
    .map(|pos| match visited.contains(&pos) {
      | true => None,
      | false => Some(visit(&grid, pos, &mut visited)),
    })
    .flatten()
    .collect::<Vec<_>>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let part_1 = compute_regions(&input)
    .iter()
    .map(|r| r.fence_cost())
    .sum::<usize>();
  output.write_part(1, &part_1);

  // TODO: part 2
}
