use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc::bootstrap;

fn parse_sentiment(line: &str) -> ((String, String), i64) {
  let parts = line.split(" ").collect::<Vec<_>>();
  let from = parts[0].to_string();
  let mut to = parts[10].to_string();
  // NOTE: Remove last period
  to.pop();
  let sentiment = parts[2];
  let units = parts[3].parse::<i64>().unwrap();
  let value = units * if sentiment == "gain" { 1 } else { -1 };

  ((from, to), value)
}

fn find_max_happiness(
  sentiments: &HashMap<(String, String), i64>,
  names: &HashSet<&String>,
) -> i64 {
  names
    .iter()
    .permutations(names.len())
    .map(|perm| {
      let first = perm.get(0).unwrap();
      let last = perm.get(perm.len() - 1).unwrap();

      perm
        .iter()
        .zip(perm.iter().skip(1))
        .chain(vec![(last, first)])
        .map(|pair| {
          let pair_1 = ((**pair.0).to_owned(), (**pair.1).to_owned());
          let pair_2 = ((**pair.1).to_owned(), (**pair.0).to_owned());
          *sentiments.get(&pair_1).unwrap_or(&0) + *sentiments.get(&pair_2).unwrap_or(&0)
        })
        .sum::<i64>()
    })
    .max()
    .unwrap_or(0)
}

fn main() {
  let (input, mut output) = bootstrap();

  let sentiments = input
    .lines()
    .map(parse_sentiment)
    .collect::<HashMap<_, _>>();
  let mut names = sentiments.iter().map(|s| &s.0.0).collect::<HashSet<_>>();
  let part_1 = find_max_happiness(&sentiments, &names);
  output.write_part(1, &part_1);

  let me = String::from("Me");
  names.insert(&me);
  let part_2 = find_max_happiness(&sentiments, &names);
  output.write_part(2, &part_2);
}
