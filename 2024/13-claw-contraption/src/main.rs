use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Machine {
  btn_a: (i64, i64),
  btn_b: (i64, i64),
  prize: (i64, i64),
}

fn parse_pos(line: &str) -> (i64, i64) {
  let parts = line.split([':', ',', '+', '=']).collect::<Vec<_>>();
  let x = parts[2].parse::<i64>().unwrap();
  let y = parts[4].parse::<i64>().unwrap();

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
struct Solution(i64, i64);

impl Solution {
  fn token(&self) -> i64 {
    3 * self.0 + self.1
  }
}

fn find_solutions(machine: &Machine) -> Vec<Solution> {
  (0..=100)
    .flat_map(|a| (0..=100).map(move |b| Solution(a, b)))
    .filter(|Solution(a, b)| {
      (
        a * machine.btn_a.0 + b * machine.btn_b.0,
        a * machine.btn_a.1 + b * machine.btn_b.1,
      ) == machine.prize
    })
    .collect()
}

fn find_best_solution(machine: &Machine, offset: (i64, i64)) -> Option<Solution> {
  let Machine {
    btn_a,
    btn_b,
    prize,
  } = machine;
  let prize = (prize.0 + offset.0, prize.1 + offset.1);

  let det = btn_a.0 * btn_b.1 - btn_a.1 * btn_b.0;
  if det == 0 {
    panic!("Found machine with zero determinant {:?}", machine);
  }

  let a_dividend = prize.0 * btn_b.1 - prize.1 * btn_b.0;
  let b_dividend = prize.1 * btn_a.0 - prize.0 * btn_a.1;

  if a_dividend % det == 0 && b_dividend % det == 0 {
    let a = a_dividend / det;
    let b = b_dividend / det;

    if a >= 0 && b >= 0 {
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
      find_solutions(&m)
        .iter()
        .map(|s| s.token())
        .min()
        .unwrap_or(0)
    })
    .sum::<i64>();
  output.write_part(1, &part_1);

  let offset = 10_000_000_000_000_i64;
  let part_2 = input
    .iter()
    .map(|m| {
      find_best_solution(&m, (offset, offset))
        .map(|s| s.token())
        .unwrap_or(0)
    })
    .sum::<i64>();
  output.write_part(2, &part_2);
}
