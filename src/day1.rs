fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
  let mut list1: Vec<i32> = vec![];
  let mut list2: Vec<i32> = vec![];
  for line in input.lines() {
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    list1.push(nums[0]);
    list2.push(nums[1]);
  }
  (list1, list2)
}

pub fn part_one(input: String) -> String {
  let (mut list1, mut list2) = parse(input);
  list1.sort();
  list2.sort();

  let mut result: i32 = 0;
  for (&x, &y) in list1.iter().zip(list2.iter()) {
    result += (y - x).abs();
  }

  result.to_string()
}

pub fn part_two(input: String) -> String {
  let (list1, list2) = parse(input);

  let mut result: i32 = 0;
  for x in list1 {
    let mut count = 0;
    for &y in list2.iter() {
      if x == y {
        count += 1;
      }
    }
    result += x * count;
  }

  result.to_string()
}
