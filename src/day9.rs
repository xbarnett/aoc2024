fn parse(input: String) -> (Vec<Option<usize>>, Vec<(usize, usize)>) {
  let mut disk = vec![];
  let mut blocks = vec![];
  for (i, c) in input.trim().chars().enumerate() {
    let n = c.to_string().parse().unwrap();
    if i % 2 == 0 {
      blocks.push((disk.len(), n));
      disk.extend((0 .. n).map(|_| Some(i / 2)));
    } else {
      disk.extend((0 .. n).map(|_| None));
    }
  }
  (disk, blocks)
}

fn checksum(disk: Vec<Option<usize>>) -> usize {
  disk.iter().enumerate().map(|(i, n)| i * n.unwrap_or(0)).sum()
}

pub fn part_one(input: String) -> String {
  let (mut disk, _) = parse(input);
  loop {
    let first_empty = disk.iter().position(|o| o.is_none()).unwrap();
    let last_full = disk.iter().rposition(|o| o.is_some()).unwrap();
    if first_empty < last_full {
      disk.swap(first_empty, last_full)
    } else {
      break;
    }
  }
  checksum(disk).to_string()
}

pub fn part_two(input: String) -> String {
  let (mut disk, mut blocks) = parse(input);
  while let Some((i, n)) = blocks.pop() {
    if let Some(j) = (0 .. i).filter(|&j| {
      (j .. j + n).all(|k| disk[k].is_none())
    }).next() {
      for k in 0 .. n {
        disk.swap(j + k, i + k);
      }
    }
  }
  checksum(disk).to_string()
}
