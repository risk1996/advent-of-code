use aoc::bootstrap;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Command {
  Mul(u64, u64),
  DoNot,
  Do,
}

fn parse_input(line: &str) -> Vec<Command> {
  let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don't)\(\)()|(do)\(\)()").unwrap();
  pattern
    .captures_iter(line)
    .map(|c| c.extract())
    .map(|(_, m)| match m {
      | ["do", _] => Command::Do,
      | ["don't", _] => Command::DoNot,
      | [x, y] => Command::Mul(x.parse::<u64>().unwrap_or(0), y.parse::<u64>().unwrap_or(0)),
    })
    .collect::<Vec<_>>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input);

  let part_1 = input
    .iter()
    .map(|c| if let Command::Mul(m, n) = c { m * n } else { 0 })
    .sum::<u64>();
  output.write_part(1, &part_1);

  let part_2 = input
    .iter()
    .fold((true, 0u64), |acc, c| match c {
      | Command::Do => (true, acc.1),
      | Command::DoNot => (false, acc.1),
      | Command::Mul(m, n) => match acc.0 {
        | true => (true, acc.1 + m * n),
        | false => (false, acc.1),
      },
    })
    .1;
  output.write_part(2, &part_2);
}
