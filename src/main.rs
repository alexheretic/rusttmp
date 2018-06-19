extern crate rusttmp_member;

use std::u64;

fn main() {
    let int: u32 = rusttmp_member::gimme_int();
    println!("{}", int);
}
