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
        n:usize,mut a:[i64;n]
    }
    a.sort();
    let mut kind: HashSet<i64> = HashSet::from_iter(a.iter().cloned());
    let ave = a.iter().sum::<i64>() / n as i64;
    kind.insert(ave);
    kind.insert(ave + 1);
    kind.insert(ave - 1);
    for i in 1..n {
        if (a[i - 1] + a[i]) % 2 == 0 {
            kind.insert((a[i - 1] + a[i]) / 2);
        } else {
            kind.insert((a[i - 1] + a[i] + 1) / 2);
            kind.insert((a[i - 1] + a[i] - 1) / 2);
            kind.insert((a[i - 1] + a[i]) / 2);
            kind.insert((a[i - 1] + a[i] + 2) / 2);
            kind.insert((a[i - 1] + a[i] - 2) / 2);
        }
    }
    let mut ans = 1e15 as i64;
    for k in kind {
        ans = ans.min(a.iter().map(|x| (*x - k).pow(2)).sum::<i64>());
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
