use std::collections::{HashMap, HashSet};

use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
  antennas: HashMap<char, Vec<(i64, i64)>>,
  bounds: (i64, i64),
}

fn parse_input(lines: &Vec<&str>) -> Input {
  let antennas = lines
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .map(move |(x, c)| (c, (x as i64, y as i64)))
    })
    .filter(|(c, _)| *c != '.')
    .fold(
      HashMap::<char, Vec<(i64, i64)>>::new(),
      |mut map, (c, (x, y))| {
        map.entry(c).or_insert(vec![]).push((x, y));

        return map;
      },
    );
  let bounds = (lines[0].len() as i64, lines.len() as i64);

  Input { antennas, bounds }
}

fn is_within_bounds(location: (i64, i64), bounds: (i64, i64)) -> bool {
  location.0 >= 0 && location.0 < bounds.0 && location.1 >= 0 && location.1 < bounds.1
}

fn get_all_antinode_locations(input: &Input) -> HashSet<(i64, i64)> {
  let mut result = HashSet::<(i64, i64)>::new();

  for (_, locations) in input.antennas.iter() {
    for i in 0..locations.len() - 1 {
      for j in (i + 1)..locations.len() {
        let i_loc = locations[i];
        let j_loc = locations[j];
        let distance = (j_loc.0 - i_loc.0, j_loc.1 - i_loc.1);

        let a1_loc = (i_loc.0 - distance.0, i_loc.1 - distance.1);
        if is_within_bounds(a1_loc, input.bounds) {
          result.insert(a1_loc);
        }

        let a2_loc = (j_loc.0 + distance.0, j_loc.1 + distance.1);
        if is_within_bounds(a2_loc, input.bounds) {
          result.insert(a2_loc);
        }
      }
    }
  }

  result
}

fn get_all_resonant_frequency_locations(input: &Input) -> HashSet<(i64, i64)> {
  let mut result = HashSet::<(i64, i64)>::new();

  for (_, locations) in input.antennas.iter() {
    for l in locations {
      result.insert(*l);
    }
  }

  for (_, locations) in input.antennas.iter() {
    for i in 0..locations.len() - 1 {
      for j in (i + 1)..locations.len() {
        let mut i_loc = locations[i];
        let mut j_loc = locations[j];
        let distance = (j_loc.0 - i_loc.0, j_loc.1 - i_loc.1);

        loop {
          let a_loc = (i_loc.0 - distance.0, i_loc.1 - distance.1);
          if is_within_bounds(a_loc, input.bounds) {
            result.insert(a_loc);
            i_loc = a_loc;
          } else {
            break;
          }
        }

        loop {
          let a_loc = (j_loc.0 + distance.0, j_loc.1 + distance.1);
          if is_within_bounds(a_loc, input.bounds) {
            result.insert(a_loc);
            j_loc = a_loc;
          } else {
            break;
          }
        }
      }
    }
  }

  result
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let part_1 = get_all_antinode_locations(&input).len();
  output.write_part(1, &part_1);

  let part_2 = get_all_resonant_frequency_locations(&input).len();
  output.write_part(2, &part_2);
}
