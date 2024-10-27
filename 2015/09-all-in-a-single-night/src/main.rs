use std::collections::{HashMap, HashSet};

use aoc::bootstrap;

fn traverse(
  costs: &HashMap<(String, String), usize>,
  locations: &HashSet<String>,
  location: &str,
  visited: &Vec<String>,
  cost: usize,
) -> Vec<(Vec<String>, usize)> {
  if visited.len() == locations.len() {
    return vec![(visited.clone(), cost)];
  }

  let mut results: Vec<(Vec<String>, usize)> = vec![];
  for l in locations {
    if visited.contains(l) || location == l {
      continue;
    }

    let mut visited = visited.clone();
    visited.push(l.to_string());

    for result in traverse(
      &costs,
      &locations,
      &l,
      &visited,
      cost + costs.get(&(location.to_string(), l.to_string())).unwrap(),
    ) {
      results.push(result);
    }
  }

  results
}

fn main() {
  let (input, mut output) = bootstrap();

  let mut locs: HashSet<String> = HashSet::new();
  let costs = input
    .split("\n")
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<_>>();
      let (from, to) = (parts[0].to_string(), parts[2].to_string());
      let cost: usize = parts[4].parse().unwrap();

      (from, to, cost)
    })
    .fold(
      HashMap::<(String, String), usize>::new(),
      |mut acc, (from, to, cost)| {
        acc.insert((from.clone(), to.clone()), cost);
        acc.insert((to.clone(), from.clone()), cost);
        locs.insert(from);
        locs.insert(to);
        acc
      },
    );

  let mut results = locs
    .iter()
    .flat_map(|l| traverse(&costs, &locs, l, &vec![l.to_string()], 0))
    .collect::<Vec<_>>();
  results.sort_by(|(_, lv), (_, rv)| lv.cmp(rv));
  let part_1 = results[0].1;
  output.write_part(1, &part_1);
  let part_2 = results.last().unwrap().1;
  output.write_part(2, &part_2);
}
