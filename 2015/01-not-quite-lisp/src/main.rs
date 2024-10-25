use aoc::bootstrap;

fn main() {
  let (input, mut output) = bootstrap();

  let mut floor = 0;
  for c in input.chars() {
    match c {
      | '(' => floor += 1,
      | ')' => floor -= 1,
      | _ => {},
    }
  }

  output.write_ln(&format!("{floor}"));
}
