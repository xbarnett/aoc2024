fn parse(input: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
  let (rules, pages) = input.split_once("\n\n").unwrap();
  let rules = rules.lines().map(|line|
    (line[0 .. 2].parse().unwrap(), line[3 .. 5].parse().unwrap())
  ).collect();
  let pages = pages.lines().map(|line|
    line.split(',').map(|n| n.parse().unwrap()).collect()
  ).collect();
  (rules, pages)
}

fn find_error(page: &[i32], rules: &[(i32, i32)]) -> Option<(usize, usize)> {
   for (x, y) in rules {
      let i = page.iter().position(|e| e == x);
      let j = page.iter().position(|e| e == y);
      if let (Some(i), Some(j)) = (i, j) {
        if i > j {
          return Some((i, j));
        }
      }
    }
    None
}

pub fn part_one(input: String) -> String {
  let (rules, pages) = parse(input);
  let mut result = 0;
  for page in pages {
    if find_error(&page, &rules).is_none() {
      result += page[page.len() / 2];
    }
  }
  result.to_string()
}

pub fn part_two(input: String) -> String {
  let (rules, pages) = parse(input);
  let mut result = 0;
  for mut page in pages {
    let mut has_error = false;
    while let Some((i, j)) = find_error(&page, &rules) {
      page.swap(i, j);
      has_error = true;
    }
    if has_error {
      result += page[page.len() / 2];
    }
  }
  result.to_string()
}
