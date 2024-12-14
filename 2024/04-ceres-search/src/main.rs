use aoc::bootstrap;

trait Reverse {
  fn reverse(&self) -> Self;
}

impl Reverse for String {
  fn reverse(&self) -> Self {
    self.chars().rev().collect()
  }
}

trait CharAt {
  fn char_at(&self, i: usize) -> Option<char>;
}

impl CharAt for &str {
  fn char_at(&self, i: usize) -> Option<char> {
    self.chars().skip(i).next()
  }
}

fn expand_input(lines: &Vec<&str>) -> Vec<String> {
  let w = lines[0].len();
  let h = lines.len();
  let h_fw = lines.iter().map(|l| String::from(*l)).collect::<Vec<_>>();
  let h_bw = h_fw.iter().map(|l| l.reverse()).collect::<Vec<_>>();
  let v_fw = (0..w)
    .map(|x| {
      (0..h)
        .map(move |y| lines[y].char_at(x).unwrap())
        .collect::<String>()
    })
    .collect::<Vec<_>>();
  let v_bw = v_fw.iter().map(|l| l.reverse()).collect::<Vec<_>>();
  let md_fw = (0..h)
    .rev()
    .map(|y| {
      (0..((h - y).min(w)))
        .map(move |l| lines[l + y].char_at(l).unwrap())
        .collect::<String>()
    })
    .chain((1..w).map(|x| {
      (0..(w - x).min(h))
        .map(|l| lines[l].char_at(x + l).unwrap())
        .collect::<String>()
    }))
    .collect::<Vec<_>>();
  let md_bw = md_fw.iter().map(|l| l.reverse()).collect::<Vec<_>>();
  let od_fw = (0..w)
    .map(|x| {
      (0..(x + 1).min(h))
        .map(move |l| lines[l].char_at(x - l).unwrap())
        .collect::<String>()
    })
    .chain((1..h).map(|y| {
      (0..(h - y).min(w))
        .map(move |l| lines[y + l].char_at(w - l - 1).unwrap())
        .collect::<String>()
    }))
    .collect::<Vec<_>>();
  let od_bw = od_fw.iter().map(|l| l.reverse()).collect::<Vec<_>>();

  [
    h_fw.as_slice(),
    h_bw.as_slice(),
    v_fw.as_slice(),
    v_bw.as_slice(),
    md_fw.as_slice(),
    md_bw.as_slice(),
    od_fw.as_slice(),
    od_bw.as_slice(),
  ]
  .concat()
}

fn count_1d_occurrence(needle: &str, haystack: &Vec<String>) -> usize {
  haystack.iter().map(|l| l.split(needle).count() - 1).sum()
}

fn chunk_input(lines: &Vec<&str>) -> Vec<[[char; 3]; 3]> {
  let w = lines[0].len();
  let h = lines.len();

  (0..w - 2)
    .flat_map(|x| {
      (0..h - 2)
        .map(|y| {
          (0..3)
            .map(|row| -> [char; 3] {
              let mut line = lines[y + row].chars().skip(x).take(3);
              let c1 = line.next().unwrap();
              let c2 = line.next().unwrap();
              let c3 = line.next().unwrap();

              [c1, c2, c3]
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn count_cross_mas(chunks: &Vec<[[char; 3]; 3]>) -> usize {
  chunks
    .iter()
    .filter(|&c| match c {
      | [['M', _, 'M'], [_, 'A', _], ['S', _, 'S']] => true,
      | [['S', _, 'M'], [_, 'A', _], ['S', _, 'M']] => true,
      | [['M', _, 'S'], [_, 'A', _], ['M', _, 'S']] => true,
      | [['S', _, 'S'], [_, 'A', _], ['M', _, 'M']] => true,
      | _ => false,
    })
    .count()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = input.split("\n").collect::<Vec<_>>();

  let expansion = expand_input(&input);
  let part_1 = count_1d_occurrence("XMAS", &expansion);
  output.write_part(1, &part_1);

  let chunks = chunk_input(&input);
  let part_2 = count_cross_mas(&chunks);
  output.write_part(2, part_2);
}
