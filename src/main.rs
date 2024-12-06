#![deny(unsafe_code)]
#![forbid(non_ascii_idents)]
#![forbid(unsafe_attr_outside_unsafe)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unused_qualifications)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::{env, io};

fn main() {
  let mut args = env::args();
  args.next();
  let day: i32 = args.next().expect("day must be given").
    parse().expect("invalid day given");
  let part: i32 = args.next().expect("part must be given").
    parse().expect("invalid part given");
  let solve: fn(String) -> String = match (day, part) {
    (1, 1) => day1::part_one,
    (1, 2) => day1::part_two,
    (2, 1) => day2::part_one,
    (2, 2) => day2::part_two,
    (3, 1) => day3::part_one,
    (3, 2) => day3::part_two,
    (4, 1) => day4::part_one,
    (4, 2) => day4::part_two,
    (5, 1) => day5::part_one,
    (5, 2) => day5::part_two,
    (6, 1) => day6::part_one,
    (6, 2) => day6::part_two,
    _ => panic!("that problem has not been solved"),
  };
  let input = io::read_to_string(io::stdin()).
    expect("failed to read input");
  let output = solve(input);
  println!("{output}");
}
