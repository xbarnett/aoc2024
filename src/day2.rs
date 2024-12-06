fn parse(input: String) -> Vec<Vec<i32>> {
  input.lines().map(|line|
    line.split_whitespace().map(|n| n.parse().unwrap()).collect()
  ).collect()
}

fn is_safe(report: &[i32]) -> bool {
  let diffs: Vec<_> = (1 .. report.len()).
    map(|i| report[i] - report[i - 1]).collect();
  diffs.iter().all(|&n| -3 <= n && n <= -1) ||
    diffs.iter().all(|&n| 1 <= n && n <= 3)
}

pub fn part_one(input: String) -> String {
  let result = parse(input).iter().filter(|v| is_safe(v)).count();
  result.to_string()
}

pub fn part_two(input: String) -> String {
  let mut result = 0;
  'outer: for report in parse(input) {
    if is_safe(&report) {
      result += 1;
      continue;
    } 
    for i in 0 .. report.len() {
      let mut removed = vec![];
      removed.extend(&report[.. i]);
      removed.extend(&report[i + 1 ..]);
      if is_safe(&removed) {
        result += 1;
        continue 'outer;
      }
    }
  }
  result.to_string()
}
