use aoc::bootstrap;

fn parse_input(lines: &Vec<&str>) -> (Vec<i64>, Vec<i64>) {
  let col_1 = lines
    .iter()
    .map(|l| l.split(" ").collect::<Vec<_>>())
    .map(|l| l.get(0).unwrap_or(&"0").parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  let col_2 = lines
    .iter()
    .map(|l| l.split(" ").collect::<Vec<_>>())
    .map(|l| l.get(l.len() - 1).unwrap_or(&"0").parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  (col_1, col_2)
}

fn main() {
  let (input, mut output) = bootstrap();
  let (mut col_1, mut col_2) = parse_input(&input.split("\n").collect::<Vec<_>>());

  col_1.sort();
  col_2.sort();

  let part_1 = (0..(col_1.len()))
    .map(|i| (col_1.get(i).unwrap_or(&0) - col_2.get(i).unwrap_or(&0)).abs())
    .sum::<i64>();
  output.write_part(1, &part_1);

  let part_2 = col_1
    .iter()
    .map(|n| n * (col_2.iter().filter(|m| *m == n).count() as i64))
    .sum::<i64>();
  output.write_part(2, &part_2);
}
