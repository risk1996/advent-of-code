use std::collections::HashSet;

use rayon::prelude::*;

use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  fn turn_right(&self) -> Self {
    match self {
      | Self::North => Self::East,
      | Self::East => Self::South,
      | Self::South => Self::West,
      | Self::West => Self::North,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
  Guard(Direction),
  Space(bool),
  Obstacle,
}

fn parse_grid(lines: &Vec<&str>) -> Vec<Vec<Cell>> {
  lines
    .iter()
    .map(|l| {
      l.chars()
        .map(|c| match c {
          | '^' => Cell::Guard(Direction::North),
          | '>' => Cell::Guard(Direction::East),
          | 'V' => Cell::Guard(Direction::South),
          | '<' => Cell::Guard(Direction::West),
          | '.' => Cell::Space(false),
          | _ => Cell::Obstacle,
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn get_guard_info(grid: &Vec<Vec<Cell>>) -> ((usize, usize), Direction) {
  let y = grid
    .iter()
    .position(|row| row.iter().any(|cell| matches!(cell, Cell::Guard(_))))
    .unwrap();
  let x = grid[y]
    .iter()
    .position(|cell| matches!(cell, Cell::Guard(_)))
    .unwrap();

  if let Cell::Guard(dir) = &grid[y][x] {
    return ((x, y), *dir);
  }

  panic!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum NextMove {
  Forward(usize, usize),
  TurnRight,
  LeaveMap,
}

fn next_move(grid: &Vec<Vec<Cell>>, direction: &Direction, position: &(usize, usize)) -> NextMove {
  let grid_bounds = (grid[0].len() - 1, grid.len() - 1);

  let next = match direction {
    | Direction::North if position.1 > 0 => NextMove::Forward(position.0, position.1 - 1),
    | Direction::East if position.0 < grid_bounds.0 => {
      NextMove::Forward(position.0 + 1, position.1)
    },
    | Direction::South if position.1 < grid_bounds.1 => {
      NextMove::Forward(position.0, position.1 + 1)
    },
    | Direction::West if position.0 > 0 => NextMove::Forward(position.0 - 1, position.1),
    | _ => NextMove::LeaveMap,
  };

  if let NextMove::Forward(x, y) = next {
    if matches!(grid[y][x], Cell::Obstacle) {
      return NextMove::TurnRight;
    }
  }

  next
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum PlayResult {
  LeaveMap,
  Loop,
}

fn play(grid: &Vec<Vec<Cell>>) -> (PlayResult, Vec<Vec<Cell>>) {
  let mut grid = grid.clone();
  let (mut guard_pos, mut guard_dir) = get_guard_info(&grid);
  let mut visited = HashSet::<((usize, usize), Direction)>::new();
  visited.insert((guard_pos, guard_dir));

  loop {
    match next_move(&grid, &guard_dir, &guard_pos) {
      | NextMove::Forward(x, y) => {
        grid[y][x] = Cell::Guard(guard_dir);
        grid[guard_pos.1][guard_pos.0] = Cell::Space(true);
        guard_pos = (x, y);
      },
      | NextMove::TurnRight => {
        guard_dir = guard_dir.turn_right();
      },
      | NextMove::LeaveMap => break (PlayResult::LeaveMap, grid),
    }

    if visited.contains(&(guard_pos, guard_dir)) {
      break (PlayResult::Loop, grid);
    }

    visited.insert((guard_pos, guard_dir));
  }
}

fn place_all_loop_obstacle_possibilities(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
  let (w, h) = (grid[0].len(), grid.len());

  (0..w)
    .into_par_iter()
    .flat_map(|x| {
      (0..h)
        .into_par_iter()
        .filter(move |y| {
          let mut mod_grid = grid.clone();

          if let Cell::Space(_) = mod_grid[*y][x] {
            mod_grid[*y][x] = Cell::Obstacle;
            let (result, _) = play(&mod_grid);

            return result == PlayResult::Loop;
          }

          false
        })
        .map(move |y| (x, y))
    })
    .collect::<Vec<_>>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let grid = parse_grid(&input.lines().collect::<Vec<_>>());

  let (_, part_1_grid) = play(&grid);
  let part_1 = part_1_grid
    .iter()
    .map(|row| {
      row
        .iter()
        .filter(|cell| matches!(cell, Cell::Guard(_) | Cell::Space(true)))
        .count()
    })
    .sum::<usize>();
  output.write_part(1, &part_1);

  let part_2 = place_all_loop_obstacle_possibilities(&grid).len();
  output.write_part(2, &part_2);
}
