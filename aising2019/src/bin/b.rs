#[allow(unused_imports)]
use itertools::*;
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
        n:usize,a:usize,b:usize,p:[usize;n]
    }
    let s = p.iter().filter(|x| **x <= a).count();
    let t = p.iter().filter(|x| a + 1 <= **x && **x <= b).count();
    let u = p.iter().filter(|x| b + 1 <= **x).count();
    println!("{}", std::cmp::min(std::cmp::min(s, t), u));
}
fn main() {
    solve();
}
