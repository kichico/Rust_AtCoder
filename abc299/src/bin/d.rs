#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use proconio::source::line::LineSource;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
use std::io::BufReader;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    let mut source = LineSource::new(BufReader::new(std::io::stdin()));
    input! {
        from &mut source,
        n:usize
    }
    let (mut left, mut right): (usize, usize) = (0, n);
    while right - left > 1 {
        let mid: usize = left + (right - left) / 2;
        println!("? {}", mid);
        input! {
            from &mut source,
            c:char
        }
        if c == '1' {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("! {}", left);
}
fn main() {
    solve();
}
