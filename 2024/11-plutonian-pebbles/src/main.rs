use aoc::bootstrap;

fn parse_input(line: &str) -> Vec<u64> {
  line
    .split(" ")
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<_>>()
}

fn blink(state: &Vec<u64>) -> Vec<u64> {
  state
    .iter()
    .flat_map(|n| {
      if *n == 0 {
        vec![1]
      } else if n.to_string().len() % 2 == 0 {
        let n = n.to_string();
        vec![
          n[0..n.len() / 2].parse::<u64>().unwrap(),
          n[n.len() / 2..n.len()].parse::<u64>().unwrap(),
        ]
      } else {
        vec![n * 2_024]
      }
    })
    .collect::<Vec<_>>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input);

  let part_1 = (0..25).fold(input.clone(), |acc, _| blink(&acc));
  output.write_part(1, &part_1.len());

  // TODO: part 2
  // NOTE part_2 is about ~43306923037450
}
