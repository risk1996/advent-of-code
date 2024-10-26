use std::fs::{metadata, remove_file, File};
use std::io::{Read, Write};

pub fn get_input(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();
  input.trim().to_string()
}

#[derive(Debug)]
pub struct Output {
  file: File,
}

pub fn get_output(filename: &str) -> Output {
  if metadata(filename).is_ok() {
    remove_file(filename).unwrap();
  }

  let file = File::create(filename).unwrap();
  Output { file }
}

impl Output {
  pub fn write(&mut self, output: &str) {
    self.file.write_all(output.as_bytes()).unwrap();
  }

  pub fn writeln(&mut self, output: &str) {
    self.write(&format!("{}\n", output));
  }
}

pub fn bootstrap() -> (String, Output) {
  let input = get_input("./input.txt");
  let output = get_output("./output.txt");
  (input, output)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_input() {
    assert_eq!(get_input("./input.txt"), "Hello, world!\n");
  }

  #[test]
  fn test_output() {
    let mut output = get_output("./output.txt");
    print!("{:?}", output);
    output.writeln("Hello, world!");
  }
}
