use aoc::bootstrap;

fn main() {
  let (input, mut output) = bootstrap();

  let mut i = 0;
  let nonce5 = loop {
    let digest = md5::compute(format!("{input}{i}"));
    let digest = format!("{:x}", digest);

    if digest.starts_with("00000") {
      break i;
    }

    i += 1;
  };

  let nonce6 = loop {
    let digest = md5::compute(format!("{input}{i}"));
    let digest = format!("{:x}", digest);

    if digest.starts_with("000000") {
      break i;
    }

    i += 1;
  };

  output.writeln(&format!(
    "Nonce (00000): {nonce5}\nNonce (000000): {nonce6}"
  ));
}
