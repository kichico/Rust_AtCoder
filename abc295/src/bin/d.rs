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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        s:Chars
    }
    let mut mp: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut cnt = vec![0; 10];
    let n = s.len();
    mp.insert(cnt.clone(), 1);
    for i in 0..n {
        let num = s[i].to_digit(10).unwrap() as usize;
        cnt[num] += 1;
        cnt[num] %= 2;
        *mp.entry(cnt.clone()).or_insert(0) += 1;
    }
    let mut ans = 0;
    for val in mp.values() {
        ans += (val * (val - 1)) / 2;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
