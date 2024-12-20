use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Machine {
  btn_a: (u64, u64),
  btn_b: (u64, u64),
  prize: (u64, u64),
}

fn parse_pos(line: &str) -> (u64, u64) {
  let parts = line.split([':', ',', '+', '=']).collect::<Vec<_>>();
  let x = parts[2].parse::<u64>().unwrap();
  let y = parts[4].parse::<u64>().unwrap();

  (x, y)
}

fn parse_input(lines: &Vec<&str>) -> Vec<Machine> {
  lines
    .chunks(4)
    .map(|chunk| {
      let btn_a = parse_pos(chunk[0]);
      let btn_b = parse_pos(chunk[1]);
      let prize = parse_pos(chunk[2]);

      Machine {
        btn_a,
        btn_b,
        prize,
      }
    })
    .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Solution(u64, u64);

impl Solution {
  fn token(&self) -> u64 {
    3 * self.0 + self.1
  }
}

fn find_best_solution(machine: &Machine, offset: (u64, u64)) -> Option<Solution> {
  let prize = (machine.prize.0 + offset.0, machine.prize.1 + offset.1);
  let upper_b = (prize.0 / machine.btn_b.0).min(prize.1 / machine.btn_b.1);

  for b in (0..=upper_b).rev() {
    let dist = (prize.0 - b * machine.btn_b.0, prize.1 - b * machine.btn_b.1);
    if dist.0 % machine.btn_a.0 == 0
      && dist.1 % machine.btn_a.1 == 0
      && dist.0 / machine.btn_a.0 == dist.1 / machine.btn_a.1
    {
      let a = dist.0 / machine.btn_a.0;
      return Some(Solution(a, b));
    }
  }

  None
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input.lines().collect::<Vec<_>>());

  let part_1 = input
    .iter()
    .map(|m| {
      find_best_solution(&m, (0, 0))
        .map(|s| s.token())
        .unwrap_or(0)
    })
    .sum::<u64>();
  output.write_part(1, &part_1);

  // TODO: part 2
}
