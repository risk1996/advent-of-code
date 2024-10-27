use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
  TurnOn,
  TurnOff,
  Toggle,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
  instruction: Instruction,
  start: (usize, usize),
  end: (usize, usize),
}

fn parse_coords(coords: &str) -> (usize, usize) {
  let coords = coords.split(",").collect::<Vec<_>>();
  let x: usize = coords[0].parse().unwrap();
  let y: usize = coords[1].parse().unwrap();

  (x, y)
}

fn parse(line: &str) -> Command {
  let instruction = if line.starts_with("turn on") {
    Instruction::TurnOn
  } else if line.starts_with("turn off") {
    Instruction::TurnOff
  } else {
    Instruction::Toggle
  };
  let mut parts = line.split(" ").collect::<Vec<_>>();
  parts.reverse();
  let end = parse_coords(parts[0]);
  let start = parse_coords(parts[2]);

  Command {
    instruction,
    start,
    end,
  }
}

fn main() {
  let (input, mut output) = bootstrap();
  let commands = input
    .split("\n")
    .map(|line| parse(line))
    .collect::<Vec<_>>();

  let mut lights = [[false; 1_000]; 1_000];
  for command in commands.iter() {
    let Command {
      instruction,
      start,
      end,
    } = command;
    for x in start.0..=end.0 {
      for y in start.1..=end.1 {
        lights[y][x] = match instruction {
          | Instruction::TurnOn => true,
          | Instruction::TurnOff => false,
          | Instruction::Toggle => !lights[y][x],
        }
      }
    }
  }
  let on_count: usize = lights
    .iter()
    .map(|row| row.iter().filter(|cell| **cell).count())
    .sum();
  output.write_part(1, &on_count);

  let mut lights = [[0u8; 1_000]; 1_000];
  for command in commands.iter() {
    let Command {
      instruction,
      start,
      end,
    } = command;
    for x in start.0..=end.0 {
      for y in start.1..=end.1 {
        lights[y][x] = lights[y][x].saturating_add_signed(match instruction {
          | Instruction::TurnOn => 1,
          | Instruction::TurnOff => -1,
          | Instruction::Toggle => 2,
        })
      }
    }
  }
  let brightness: usize = lights
    .iter()
    .map(|row| row.iter().map(|cell| *cell as usize).sum::<usize>())
    .sum();
  output.write_part(2, &brightness);
}
