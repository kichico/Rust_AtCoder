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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        mut s:Chars
    }
    s.reverse();
    let mut s = s.iter().collect::<String>();
    let dic = vec!["00", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut ans = 0;
    while !s.is_empty() {
        'outer: for d in &dic {
            if s.ends_with(*d) {
                for i in 0..d.len() {
                    s.pop();
                }
                ans += 1;
                break 'outer;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
