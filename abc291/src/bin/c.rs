#[allow(unused_imports)]
use itertools::*;
use maplit::hashset;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:Chars
    }
    let mut now = hashset! {(0,0)};
    let mut x = 0;
    let mut y = 0;
    for c in s {
        if c == 'U' {
            y += 1;
        } else if c == 'D' {
            y -= 1;
        } else if c == 'R' {
            x += 1;
        } else {
            x -= 1;
        }
        if now.contains(&(x, y)) {
            println!("Yes");
            return;
        }
        now.insert((x, y));
    }
    println!("No");
}
fn main() {
    solve();
}
