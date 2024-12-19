use std::collections::HashMap;

use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq)]
struct Input {
  pub rules: Vec<[u64; 2]>,
  pub queues: Vec<Vec<u64>>,
}

fn parse_input(lines: &Vec<&str>) -> Input {
  let (rules, queues) = lines.split_at(lines.iter().position(|l| *l == "").unwrap() + 1);
  let mut rules = rules.iter().collect::<Vec<_>>();
  rules.pop();
  let rules = rules
    .iter()
    .map(|r| {
      r.split("|")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
    })
    .collect::<Vec<_>>();
  let queues = queues
    .iter()
    .map(|q| {
      q.split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  Input { rules, queues }
}

fn find_queue_violations<'a>(rules: &'a Vec<[u64; 2]>, queue: &Vec<u64>) -> Vec<&'a [u64; 2]> {
  let map = queue
    .iter()
    .enumerate()
    .map(|(i, p)| (p, i))
    .collect::<HashMap<_, _>>();

  rules
    .iter()
    .filter(|[p, s]| match (map.get(p), map.get(s)) {
      | (Some(p), Some(s)) => p > s,
      | _ => false,
    })
    .collect::<Vec<_>>()
}

fn reorder_queue(violations: &Vec<&[u64; 2]>, queue: &Vec<u64>) -> Vec<u64> {
  let mut queue = queue.clone();

  for [p, s] in violations {
    let pi = queue.iter().position(|n| n == p).unwrap();
    let si = queue.iter().position(|n| n == s).unwrap();

    if pi < si {
      continue;
    }

    queue.remove(si);
    queue.insert(pi, *s);
  }

  queue
}

fn midpoint(len: usize) -> usize {
  (len + 1) / 2 - 1
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let processed_input = input
    .queues
    .iter()
    .map(|q| (q, find_queue_violations(&input.rules, &q)))
    .collect::<Vec<_>>();

  let part_1 = processed_input
    .iter()
    .filter(|(_, violations)| violations.len() == 0)
    .map(|(q, _)| q[midpoint(q.len())])
    .sum::<u64>();
  output.write_part(1, &part_1);

  let part_2 = processed_input
    .iter()
    .filter(|(_, violations)| violations.len() > 0)
    .map(|(queue, violations)| {
      let mut violations = violations.clone();
      let mut queue = (*queue).clone();

      loop {
        queue = reorder_queue(&violations, &queue);
        violations = find_queue_violations(&input.rules, &queue);

        if violations.len() == 0 {
          break;
        }
      }

      queue
    })
    .map(|q| q[midpoint(q.len())])
    .sum::<u64>();
  output.write_part(2, &part_2);
}
