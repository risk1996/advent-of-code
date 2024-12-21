use std::collections::HashSet;

use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Robot {
  position: (i64, i64),
  velocity: (i64, i64),
}

fn parse_input(line: &str) -> Robot {
  let parts = line.split(['=', ',', ' ']).collect::<Vec<_>>();
  let position = (
    parts[1].parse::<i64>().unwrap(),
    parts[2].parse::<i64>().unwrap(),
  );
  let velocity = (
    parts[4].parse::<i64>().unwrap(),
    parts[5].parse::<i64>().unwrap(),
  );

  Robot { position, velocity }
}

fn get_position_after(robot: &Robot, step: &i64, bounds: &(i64, i64)) -> (i64, i64) {
  (
    ((robot.position.0 + step * robot.velocity.0) % bounds.0 + bounds.0) % bounds.0,
    ((robot.position.1 + step * robot.velocity.1) % bounds.1 + bounds.1) % bounds.1,
  )
}

fn get_safety_factor(positions: &Vec<(i64, i64)>, bounds: &(i64, i64)) -> i64 {
  let center = (bounds.0 / 2, bounds.1 / 2);
  positions
    .iter()
    .fold([0; 4], |mut q, pos| {
      match pos {
        | (x, y) if *x > center.0 && *y < center.1 => q[0] += 1,
        | (x, y) if *x > center.0 && *y > center.1 => q[1] += 1,
        | (x, y) if *x < center.0 && *y > center.1 => q[2] += 1,
        | (x, y) if *x < center.0 && *y < center.1 => q[3] += 1,
        | _ => (),
      }

      q
    })
    .iter()
    .product::<i64>()
}

fn remove_blob(positions: &mut HashSet<(i64, i64)>, start: &(i64, i64)) -> i64 {
  if positions.remove(start) {
    return [
      (start.0, start.1 - 1),
      (start.0 + 1, start.1),
      (start.0, start.1 + 1),
      (start.0 - 1, start.1),
    ]
    .map(|next| remove_blob(positions, &next))
    .iter()
    .sum::<i64>()
      + 1;
  }

  0
}

fn get_largest_blob_size(positions: &HashSet<(i64, i64)>) -> i64 {
  let mut positions = positions.clone();
  let mut result = 0;

  while positions.len() > 0 {
    let start = positions.iter().next().unwrap().clone();
    let blob = remove_blob(&mut positions, &start);

    if blob > result {
      result = blob;
    }
  }

  result
}

fn print_grid(positions: &HashSet<(i64, i64)>, bounds: &(i64, i64), step: &i64) -> String {
  let result = format!("--------- Step {:03} ---------", step);
  let grid = (0..bounds.1)
    .map(|y| {
      (0..bounds.0)
        .map(move |x| {
          let contains = positions.contains(&(x, y));
          if contains { "*" } else { " " }
        })
        .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join("\n");

  [result, grid].join("\n")
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = input.lines().map(|l| parse_input(l)).collect::<Vec<_>>();

  let bounds = (101, 103);
  let steps = 100;
  let part_1 = get_safety_factor(
    &input
      .iter()
      .map(|r| get_position_after(&r, &steps, &bounds))
      .collect::<Vec<_>>(),
    &bounds,
  );
  output.write_part(1, &part_1);

  for step in 0..i64::MAX {
    let positions = input
      .iter()
      .map(|r| get_position_after(&r, &step, &bounds))
      .collect::<HashSet<_>>();
    let blob_size = get_largest_blob_size(&positions);
    println!("Step {}: {}", step, blob_size);

    if blob_size > 100 {
      output.write_part(2, &step);
      output.write(&format!("{}\n\n", print_grid(&positions, &bounds, &step,)));
      break;
    }
  }
}
