use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
  Wall,
  Box,
  RightBox,
  LeftBox,
  Space,
  Robot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

fn parse_grid(lines: &str) -> Vec<Vec<Cell>> {
  lines
    .lines()
    .map(|l| {
      l.chars()
        .map(|c| match c {
          | '#' => Some(Cell::Wall),
          | '.' => Some(Cell::Space),
          | 'O' => Some(Cell::Box),
          | '@' => Some(Cell::Robot),
          | _ => None,
        })
        .flatten()
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn expand_grid(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
  grid
    .iter()
    .map(|line| {
      line
        .iter()
        .flat_map(|c| match c {
          | Cell::Robot => vec![Cell::Robot, Cell::Space],
          | Cell::Box | Cell::RightBox | Cell::LeftBox => vec![Cell::LeftBox, Cell::RightBox],
          | cell => vec![*cell, *cell],
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn find_positions(grid: &Vec<Vec<Cell>>, cell: &Cell) -> Vec<(usize, usize)> {
  let bounds = (grid[0].len(), grid.len());
  let mut result = vec![];

  for y in 0..bounds.1 {
    for x in 0..bounds.0 {
      if grid[y][x] == *cell {
        result.push((x, y));
      }
    }
  }

  result
}

fn parse_directions(line: &str) -> Vec<Direction> {
  line
    .chars()
    .map(|c| match c {
      | '^' => Some(Direction::Up),
      | '>' => Some(Direction::Right),
      | 'v' => Some(Direction::Down),
      | '<' => Some(Direction::Left),
      | _ => None,
    })
    .flatten()
    .collect::<Vec<_>>()
}

fn get_target(pos: &(usize, usize), dir: &Direction) -> (usize, usize) {
  match dir {
    | Direction::Up => (pos.0, pos.1 - 1),
    | Direction::Right => (pos.0 + 1, pos.1),
    | Direction::Down => (pos.0, pos.1 + 1),
    | Direction::Left => (pos.0 - 1, pos.1),
  }
}

fn can_act(grid: &mut Vec<Vec<Cell>>, pos: &(usize, usize), dir: &Direction) -> bool {
  let target = get_target(pos, dir);

  match grid[target.1][target.0] {
    | Cell::Wall => false,
    | Cell::Box => can_act(grid, &target, dir),
    | Cell::RightBox => match dir {
      | Direction::Up | Direction::Down => {
        can_act(grid, &(target.0 - 1, target.1), dir) && can_act(grid, &target, dir)
      },
      | Direction::Right => can_act(grid, &target, dir),
      | Direction::Left => can_act(grid, &(target.0 - 1, target.1), dir),
    },
    | Cell::LeftBox => match dir {
      | Direction::Up | Direction::Down => {
        can_act(grid, &target, dir) && can_act(grid, &(target.0 + 1, target.1), dir)
      },
      | Direction::Right => can_act(grid, &(target.0 + 1, target.1), dir),
      | Direction::Left => can_act(grid, &target, dir),
    },
    | Cell::Space => true,
    | Cell::Robot => false,
  }
}

fn act(grid: &mut Vec<Vec<Cell>>, pos: &(usize, usize), dir: &Direction) -> (usize, usize) {
  let target = get_target(pos, dir);

  match grid[target.1][target.0] {
    | Cell::Wall => *pos,
    | Cell::Box => {
      if can_act(grid, &target, dir) {
        act(grid, &target, dir);
        (grid[target.1][target.0], grid[pos.1][pos.0]) =
          (grid[pos.1][pos.0], grid[target.1][target.0]);
        target
      } else {
        *pos
      }
    },
    | Cell::RightBox => {
      if can_act(grid, &(target.0 - 1, target.1), dir) && can_act(grid, &target, dir) {
        if *dir == Direction::Left {
          act(grid, &(target.0 - 1, target.1), dir);
          (
            grid[target.1][target.0 - 1],
            grid[target.1][target.0],
            grid[pos.1][pos.0],
          ) = (
            grid[target.1][target.0],
            grid[pos.1][pos.0],
            grid[target.1][target.0 - 1],
          );
        } else {
          act(grid, &target, dir);
          (grid[target.1][target.0], grid[pos.1][pos.0]) =
            (grid[pos.1][pos.0], grid[target.1][target.0]);

          if *dir != Direction::Right {
            act(grid, &(target.0 - 1, target.1), dir);
          }
          (grid[target.1][target.0 - 1], grid[pos.1][pos.0]) =
            (grid[pos.1][pos.0], grid[target.1][target.0 - 1]);
        }
        target
      } else {
        *pos
      }
    },
    | Cell::LeftBox => {
      if can_act(grid, &target, dir) && can_act(grid, &(target.0 + 1, target.1), dir) {
        if *dir == Direction::Right {
          act(grid, &(target.0 + 1, target.1), dir);
          (
            grid[target.1][target.0 + 1],
            grid[target.1][target.0],
            grid[pos.1][pos.0],
          ) = (
            grid[target.1][target.0],
            grid[pos.1][pos.0],
            grid[target.1][target.0 + 1],
          );
        } else {
          act(grid, &target, dir);
          (grid[target.1][target.0], grid[pos.1][pos.0]) =
            (grid[pos.1][pos.0], grid[target.1][target.0]);

          if *dir != Direction::Left {
            act(grid, &(target.0 + 1, target.1), dir);
          }
          (grid[target.1][target.0 + 1], grid[pos.1][pos.0]) =
            (grid[pos.1][pos.0], grid[target.1][target.0 + 1]);
        }
        target
      } else {
        *pos
      }
    },
    | Cell::Space => {
      (grid[target.1][target.0], grid[pos.1][pos.0]) =
        (grid[pos.1][pos.0], grid[target.1][target.0]);
      target
    },
    | Cell::Robot => panic!(),
  }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Cell>>) {
  for line in grid {
    println!(
      "{}",
      line
        .iter()
        .map(|c| match c {
          | Cell::Wall => '#',
          | Cell::Box => 'O',
          | Cell::RightBox => ']',
          | Cell::LeftBox => '[',
          | Cell::Space => '.',
          | Cell::Robot => '@',
        })
        .collect::<String>()
    )
  }
}

fn gps(pos: &(usize, usize)) -> usize {
  pos.0 + 100 * pos.1
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = input.split("\n\n").collect::<Vec<_>>();

  let directions = parse_directions(input[1]);

  let grid = parse_grid(input[0]);
  let robot_pos = find_positions(&grid, &Cell::Robot)[0];

  let part_1 = find_positions(
    &directions
      .iter()
      .fold((grid.clone(), robot_pos), |(mut grid, robot_pos), dir| {
        let next_pos = act(&mut grid, &robot_pos, dir);
        (grid, next_pos)
      })
      .0,
    &Cell::Box,
  )
  .iter()
  .map(|pos| gps(&pos))
  .sum::<usize>();
  output.write_part(1, &part_1);

  let grid = expand_grid(&grid);
  let robot_pos = find_positions(&grid, &Cell::Robot)[0];

  let part_2 = find_positions(
    &directions
      .iter()
      .fold((grid.clone(), robot_pos), |(mut grid, robot_pos), dir| {
        let next_pos = act(&mut grid, &robot_pos, dir);
        (grid, next_pos)
      })
      .0,
    &Cell::LeftBox,
  )
  .iter()
  .map(|pos| gps(&pos))
  .sum::<usize>();
  output.write_part(2, &part_2);
}
