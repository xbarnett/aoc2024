#![deny(unsafe_code)]
#![forbid(non_ascii_idents)]
#![forbid(unsafe_attr_outside_unsafe)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unused_qualifications)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
  println!("hi!");
}
