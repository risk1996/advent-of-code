use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Sector {
  File { id: usize, length: usize },
  Empty { length: usize },
}

impl Sector {
  fn len(&self) -> usize {
    match self {
      | Sector::File { length, .. } => *length,
      | Sector::Empty { length } => *length,
    }
  }
}

fn parse_input(line: &str) -> Vec<Sector> {
  let mut id = 0;

  line
    .chars()
    .map(|c| c.to_string().parse::<usize>().unwrap())
    .enumerate()
    .map(|(i, length)| match i % 2 == 0 {
      | true => {
        let f = Sector::File { id, length };
        id += 1;
        f
      },
      | false => Sector::Empty { length },
    })
    .collect::<Vec<_>>()
}

fn expand(disk_map: &Vec<Sector>) -> Vec<Option<usize>> {
  let size = disk_map.iter().map(|s| s.len()).sum::<usize>();
  let mut result: Vec<Option<usize>> = vec![None; size];

  let mut index = 0;
  for sector in disk_map {
    if let Sector::File { id, length } = sector {
      for i in 0..*length {
        result[index + i] = Some(*id);
      }
    }

    index += sector.len();
  }

  result
}

fn compact_fragment(disk: &Vec<Option<usize>>) -> Vec<Option<usize>> {
  let mut disk = disk.clone();
  let mut head = disk
    .iter()
    .position(|id| *id == None)
    .unwrap_or(disk.len() - 1);
  let mut tail = disk.len() - 1;

  loop {
    while matches!(disk[head], Some(_)) {
      head += 1;
    }

    while matches!(disk[tail], None) {
      tail -= 1;
    }

    if head >= tail {
      break;
    }

    disk[head] = disk[tail].clone();
    disk[tail] = None;
  }

  disk
}

fn count_contiguous(disk: &Vec<Option<usize>>, index: usize, forward: bool) -> usize {
  let value = disk[index];
  let mut length = 0;
  let mut index = index;

  while index > 0 && index < disk.len() && disk[index] == value {
    length += 1;
    index = if forward { index + 1 } else { index - 1 }
  }

  length
}

fn find_empty(disk: &Vec<Option<usize>>, length: usize, tail: usize) -> Option<usize> {
  let mut i = 0;

  loop {
    if matches!(disk[i], None) {
      let empty_length = count_contiguous(disk, i, true);

      if empty_length >= length {
        return Some(i);
      } else {
        i += empty_length - 1;
      }
    }

    i += 1;

    if i >= tail {
      break;
    }
  }

  None
}

fn defrag(disk: &Vec<Option<usize>>) -> Vec<Option<usize>> {
  let mut disk = disk.clone();
  let mut tail = disk.len() - 1;

  loop {
    if tail == 0 {
      break;
    }

    while matches!(disk[tail], None) {
      tail -= 1;
    }

    if let Some(file_id) = disk[tail] {
      let file_length = count_contiguous(&disk, tail, false);
      let empty_index = find_empty(&disk, file_length, tail);

      if let Some(empty_index) = empty_index {
        for i in 0..file_length {
          disk[empty_index + i] = Some(file_id);
          disk[tail - i] = None;
        }
      } else {
        tail -= file_length - 1;
      }

      tail -= 1;
    }
  }

  disk
}

fn checksum(disk: &Vec<Option<usize>>) -> usize {
  disk
    .iter()
    .enumerate()
    .map(|(i, id)| i * id.unwrap_or(0))
    .sum::<usize>()
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = parse_input(&input);

  let disk = expand(&input);

  let part_1 = checksum(&compact_fragment(&disk));
  output.write_part(1, &part_1);

  let part_2 = checksum(&defrag(&disk));
  output.write_part(2, &part_2);
}
