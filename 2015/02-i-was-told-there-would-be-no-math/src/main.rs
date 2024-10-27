use aoc::bootstrap;

fn main() {
  let (input, mut output) = bootstrap();

  let mut area = 0;
  let mut ribbon = 0;
  for line in input.split("\n") {
    if line == "" {
      continue;
    }

    let mut measurements = line
      .split("x")
      .map(|v| v.parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    measurements.sort();
    let x = measurements[0];
    let y = measurements[1];
    let z = measurements[2];

    let a = x * y;
    let b = y * z;
    let c = x * z;

    let slack = a.min(b).min(c);

    area += 2 * (a + b + c) + slack;
    ribbon += 2 * (x + y) + (x * y * z);
  }

  output.write_part(1, &area);
  output.write_part(2, &ribbon);
}
