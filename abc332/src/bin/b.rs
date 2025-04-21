#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
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
        k:usize, glass_size:usize,mug_size:usize
    }
    let mut glass = 0;
    let mut mug = 0;
    for _i in 0..k {
        if glass == glass_size {
            glass = 0;
        } else if mug == 0 {
            mug = mug_size;
        } else {
            let w = mug.min(glass_size - glass);
            glass += w;
            mug -= w;
        }
        //println!("{}:{} {}", _i, glass, mug);
    }
    println!("{} {}", glass, mug);
}
fn main() {
    solve();
}
