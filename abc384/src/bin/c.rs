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
        haiten:[usize;5]
    }
    let mut ans: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
    for i in 1..=5 {
        for v in (0..5usize).into_iter().combinations(i) {
            let score = v.iter().map(|x| haiten[*x]).sum::<usize>();
            let name = v
                .iter()
                .map(|x| ('A' as u8 + *x as u8) as char)
                .collect::<String>();
            let e = ans.entry(score).or_insert(BTreeSet::new());
            e.insert(name);
        }
    }
    for (_score, set) in ans.iter().rev() {
        for name in set {
            println!("{}", name);
        }
    }
}
fn main() {
    solve();
}
