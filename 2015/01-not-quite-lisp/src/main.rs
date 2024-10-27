use aoc::bootstrap;

fn main() {
  let (input, mut output) = bootstrap();

  let mut floor = 0;
  let mut basement = std::usize::MAX;
  for (i, c) in input.chars().enumerate() {
    match c {
      | '(' => floor += 1,
      | ')' => floor -= 1,
      | _ => {},
    }

    if floor < 0 && (i + 1) < basement {
      basement = i + 1
    }
  }

  output.write_part(1, &floor);
  output.write_part(2, &basement);
}
