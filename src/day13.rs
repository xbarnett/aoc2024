fn parse_nums(line: &str, before: char) -> (i64, i64) {
  let (a, b) = line.split_once(",").unwrap();
  let (_, a) = a.split_once(before).unwrap();
  let (_, b) = b.split_once(before).unwrap();
  (a.parse().unwrap(), b.parse().unwrap())
}

fn solve(input: String, error: i64) -> String {
  let mut result = 0;
  for machine in input.split("\n\n") {
    let mut lines = machine.lines();
    let (a1, a2) = parse_nums(lines.next().unwrap(), '+');
    let (b1, b2) = parse_nums(lines.next().unwrap(), '+');
    let (p1, p2) = parse_nums(lines.next().unwrap(), '=');
    let (p1, p2) = (p1 + error, p2 + error);
    let det = a1 * b2 - a2 * b1;
    let a = b2 * p1 - b1 * p2;
    let b = a1 * p2 - a2 * p1;
    if a % det == 0 && b % det == 0 {
      result += (3 * a + b) / det;
    }
  }
  result.to_string()
}

pub fn part_one(input: String) -> String {
  solve(input, 0)
}

pub fn part_two(input: String) -> String {
  solve(input, 10_000_000_000_000)
}
