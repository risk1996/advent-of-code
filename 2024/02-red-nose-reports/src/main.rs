use aoc::bootstrap;

fn parse_line(line: &str) -> Vec<i64> {
  line
    .split(" ")
    .map(|n| n.parse::<i64>().unwrap_or(0))
    .collect::<Vec<_>>()
}

fn is_report_safe(report: &Vec<i64>) -> bool {
  if report.len() < 2 {
    return true;
  }

  let first = report.get(0).unwrap();
  let last = report.get(report.len() - 1).unwrap();
  let is_increasing = first < last;

  report.iter().zip(report.iter().skip(1)).all(|(l, r)| {
    (if is_increasing { l < r } else { l > r }) && (*l - *r).abs() >= 1 && (*l - *r).abs() <= 3
  })
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = input.split("\n").map(parse_line).collect::<Vec<_>>();

  let part_1 = input.iter().filter(|r| is_report_safe(*r)).count();
  output.write_part(1, &part_1);

  let part_2 = input
    .iter()
    .filter(|r| {
      if is_report_safe(*r) {
        return true;
      }

      (0..(r.len())).any(|i| {
        let mut r = (*r).clone();
        r.remove(i);
        is_report_safe(&r)
      })
    })
    .count();
  output.write_part(2, &part_2);
}
