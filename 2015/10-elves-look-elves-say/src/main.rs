use aoc::bootstrap;

fn run_length_encoding(str: &str) -> String {
  if str.len() == 0 {
    return String::new();
  }

  let mut result = String::new();
  let mut char: char = str.chars().next().unwrap();
  let mut length: usize = 0;

  for c in str.chars() {
    if c == char {
      length += 1;
    } else {
      result.push_str(&format!("{length}{char}"));
      char = c;
      length = 1;
    }
  }

  result.push_str(&format!("{length}{char}"));

  result
}

fn main() {
  let (input, mut output) = bootstrap();

  let part_1 = (0..40)
    .fold(input.clone(), |acc, _| run_length_encoding(&acc))
    .len();
  output.write_part(1, part_1);

  let part_2 = (0..50)
    .fold(input.clone(), |acc, _| run_length_encoding(&acc))
    .len();
  output.write_part(2, part_2);
}
