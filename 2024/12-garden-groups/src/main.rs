use std::{collections::HashSet, ops::Add};

use aoc::bootstrap;

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<char>> {
  lines
    .iter()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
  label: char,
  positions: HashSet<(usize, usize)>,
  circumference: usize,
}

impl Add for Region {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let label = self.label;
    let positions = self
      .positions
      .union(&rhs.positions)
      .map(|p| *p)
      .collect::<HashSet<_, _>>();
    let circumference = self.circumference + rhs.circumference;

    Region {
      label,
      positions,
      circumference,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn all() -> [Direction; 4] {
    [
      Direction::Up,
      Direction::Right,
      Direction::Down,
      Direction::Left,
    ]
  }
}

fn step(pos: &(usize, usize), direction: &Direction) -> Option<(usize, usize)> {
  match direction {
    | Direction::Up if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
    | Direction::Right => Some((pos.0 + 1, pos.1)),
    | Direction::Down => Some((pos.0, pos.1 + 1)),
    | Direction::Left if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
    | _ => None,
  }
}

fn get_surroundings(pos: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
  Direction::all()
    .iter()
    .map(|dir| step(&pos, &dir))
    .flatten()
    .filter(|&next| next.0 < bounds.0 && next.1 < bounds.1)
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
    positions: HashSet::from([pos]),
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

fn get_border_directions(
  positions: &HashSet<(usize, usize)>,
  pos: (usize, usize),
) -> Vec<Direction> {
  Direction::all()
    .iter()
    .map(|dir| (dir, step(&pos, &dir)))
    .filter(|(_, step)| step.map(|s| !positions.contains(&s)).unwrap_or(true))
    .map(|(dir, _)| *dir)
    .collect::<Vec<_>>()
}

fn clear_side_border(
  borders: &mut Vec<((usize, usize), Direction)>,
  pivot: &((usize, usize), Direction),
) {
  let dir = pivot.1;
  let neighbors_directions = if matches!(dir, Direction::Up | Direction::Down) {
    [Direction::Left, Direction::Right]
  } else {
    [Direction::Up, Direction::Down]
  };

  for nd in neighbors_directions {
    let mut neighbor = pivot.0.clone();

    loop {
      if let Some(next_neighbor) = step(&neighbor, &nd) {
        neighbor = next_neighbor;

        if let Some(index) = borders.iter().position(|b| *b == (neighbor, dir)) {
          borders.remove(index);
        } else {
          break;
        }
      } else {
        break;
      }
    }
  }
}

fn compute_sides(positions: &HashSet<(usize, usize)>) -> usize {
  let mut borders = positions
    .iter()
    .flat_map(|pos| {
      get_border_directions(positions, *pos)
        .iter()
        .map(|dir| (*pos, *dir))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let mut sides = 0;
  while borders.len() > 0 {
    sides += 1;
    let pivot = borders.pop().unwrap();
    clear_side_border(&mut borders, &pivot);
  }

  sides
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let regions = compute_regions(&input);

  let part_1 = regions
    .iter()
    .map(|r| r.positions.len() * r.circumference)
    .sum::<usize>();
  output.write_part(1, &part_1);

  let part_2 = regions
    .iter()
    .map(|r| r.positions.len() * compute_sides(&r.positions))
    .sum::<usize>();
  output.write_part(2, &part_2);
}
