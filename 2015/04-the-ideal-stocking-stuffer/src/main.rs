use aoc::bootstrap;

fn main() {
  let (input, mut output) = bootstrap();

  let mut i = 0;
  let nonce_5 = loop {
    let digest = md5::compute(format!("{input}{i}"));
    let digest = format!("{:x}", digest);

    if digest.starts_with("00000") {
      break i;
    }

    i += 1;
  };
  output.write_part(1, &nonce_5);

  let nonce_6 = loop {
    let digest = md5::compute(format!("{input}{i}"));
    let digest = format!("{:x}", digest);

    if digest.starts_with("000000") {
      break i;
    }

    i += 1;
  };
  output.write_part(2, &nonce_6);
}
