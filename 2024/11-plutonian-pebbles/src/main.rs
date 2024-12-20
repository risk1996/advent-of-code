use std::collections::HashMap;

use aoc::bootstrap;

fn parse_input(line: &str) -> Vec<usize> {
  line
    .split(" ")
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<_>>()
}

fn blink(n: usize) -> Vec<usize> {
  match n {
    | 0 => vec![1],
    | n if n.to_string().len() % 2 == 0 => {
      let n = n.to_string();
      vec![
        n[0..n.len() / 2].parse::<usize>().unwrap(),
        n[n.len() / 2..n.len()].parse::<usize>().unwrap(),
      ]
    },
    | n => vec![n * 2_024],
  }
}

fn blink_cache(cache: &mut HashMap<(usize, usize), usize>, n: usize, steps: usize) -> usize {
  if steps == 0 {
    return 1;
  }

  if let Some(v) = cache.get(&(n, steps)) {
    return *v;
  }

  let next = blink(n);
  cache.entry((n, 1)).insert_entry(next.len());

  next
    .iter()
    .map(|v| {
      let result = blink_cache(cache, *v, steps - 1);
      cache.entry((*v, steps - 1)).insert_entry(result);
      result
    })
    .sum::<usize>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input);

  let mut cache = HashMap::<(usize, usize), usize>::new();

  let part_1 = input
    .iter()
    .map(|n| blink_cache(&mut cache, *n, 25))
    .sum::<usize>();
  output.write_part(1, &part_1);

  let part_2 = input
    .iter()
    .map(|n| blink_cache(&mut cache, *n, 75))
    .sum::<usize>();
  output.write_part(2, &part_2);
}
