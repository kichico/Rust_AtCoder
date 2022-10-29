#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,mut a:[i64;n]
    }
    a.sort();
    let mut cand = vec![a[n - 3], a[n - 2], a[n - 1]];
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    for v in cand.iter().permutations(3) {
        let mut str = "".to_string();
        for i in 0..3 {
            str += &v[i].to_string();
        }
        ans.insert(str.parse().unwrap());
    }
    println!("{}", ans.iter().next_back().unwrap());
}

fn main() {
    solve();
}
