#![deny(unsafe_code)]
#![forbid(non_ascii_idents)]
#![forbid(unsafe_attr_outside_unsafe)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unused_qualifications)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod day1;

use std::{env, io};

fn main() {
  let mut args = env::args();
  args.next();
  let day: i32 = args.next().expect("day must be given").
    parse().expect("invalid day given");
  let part: i32 = args.next().expect("part must be given").
    parse().expect("invalid part given");
  let solve: fn(String, i32) -> String = match day {
    1 => day1::solve,
    _ => panic!("that day has not been solved"),
  };
  let input = io::read_to_string(io::stdin()).
    expect("failed to read input");
  let output = solve(input, part);
  println!("{output}");
}
