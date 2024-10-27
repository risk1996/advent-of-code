use std::collections::HashSet;

use aoc::bootstrap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Pos {
  pub fn act(&mut self, char: char) {
    match char {
      | '^' => self.1 += 1,
      | 'v' => self.1 -= 1,
      | '<' => self.0 -= 1,
      | '>' => self.0 += 1,
      | _ => {},
    }
  }
}

fn main() {
  let (input, mut output) = bootstrap();

  // Part 1
  let mut position = Pos(0, 0);
  let mut visited = HashSet::<Pos>::new();
  visited.insert(position);

  for c in input.chars() {
    position.act(c);
    visited.insert(position);
  }

  let part_1 = visited.len();
  output.write_part(1, &part_1);

  // Part 2
  let mut positions = (Pos(0, 0), Pos(0, 0));
  visited.clear();
  visited.insert(positions.1);

  for (i, c) in input.chars().enumerate() {
    if i % 2 == 0 {
      positions.0.act(c);
      visited.insert(positions.0);
    } else {
      positions.1.act(c);
      visited.insert(positions.1);
    }
  }

  let part_2 = visited.len();
  output.write_part(2, &part_2);
}
