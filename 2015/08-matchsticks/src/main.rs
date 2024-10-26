use aoc::bootstrap;

fn get_memory_string(line: &str) -> String {
  line[1..(line.len() - 1)]
    .to_string()
    // NOTE: Replaces '\\' with '?' so that it does not get rematched on the next iteration
    .replace("\\\\", "?")
    .replace("\\\"", "\"")
    .split("\\x")
    .enumerate()
    .map(|(i, part)| {
      if i == 0 {
        part.to_string()
      } else {
        part[1..].to_string()
      }
    })
    .collect::<Vec<_>>()
    .join("")
}

fn get_representation_string(line: &str) -> String {
  let result = line.replace("\\", "\\\\").replace("\"", "\\\"");
  format!("\"{result}\"")
}

fn main() {
  let (input, mut output) = bootstrap();

  let part_1: usize = input
    .split("\n")
    .map(|line| line.len() - get_memory_string(line).len())
    .sum();
  let part_2: usize = input
    .split("\n")
    .map(|line| get_representation_string(line).len() - line.len())
    .sum();

  output.writeln(&format!("Part 1: {part_1}\nPart 2: {part_2}"));
}
