#[allow(unused_imports)]
use itertools::Itertools;
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
        n:usize,m:usize,mut six:[Chars;n],three:[Chars;m]
    }
    let mut dic: HashSet<Vec<char>> = HashSet::new();
    for s in three {
        let mut ss = s.clone();
        dic.insert(ss);
    }
    let mut ans = 0;
    for s in six {
        let mut ss = s.clone();
        ss.reverse();
        for _ in 0..3 {
            ss.pop();
        }
        ss.reverse();
        if dic.contains(&ss) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
