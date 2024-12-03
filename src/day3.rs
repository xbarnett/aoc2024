fn parse_int(input: &str) -> Option<(u32, &str)> {
  let digits: Vec<u32> = input.chars().map_while(|c| c.to_digit(10)).collect();
  if digits.is_empty() {
    return None;
  }

  let mut result = 0;
  for i in &digits {
    result = result * 10 + i;
  }
  Some((result, &input[digits.len() ..]))
}

fn parse_mul(input: &str) -> Option<(u32, &str)> {
  let input = input.strip_prefix("mul(")?;
  let (x, input) = parse_int(input)?;
  let input = input.strip_prefix(",")?;
  let (y, input) = parse_int(input)?;
  let input = input.strip_prefix(")")?;

  Some((x * y, input))
}

pub fn part_one(input: String) -> String {
  let mut input  = input.as_str();
  let mut result = 0;
  while !input.is_empty() {
    input = match parse_mul(input) {
      Some((n, input)) => { result += n; input },
      None => &input[1 ..],
    };
  }
  result.to_string()
}

pub fn part_two(input: String) -> String {
  let mut input  = input.as_str();
  let mut result = 0;
  let mut active = true;
  while !input.is_empty() {
    if input.starts_with("do()") { active = true; }
    if input.starts_with("don't()") { active = false; }
    input = match parse_mul(input) {
      Some((n, input)) => { if active { result += n; } input },
      None => &input[1 ..],
    };
  }
  result.to_string()
}
