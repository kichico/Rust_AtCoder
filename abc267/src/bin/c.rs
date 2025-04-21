#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
        n:usize,m:usize,a:[i64;n]
    }
    let mut que: VecDeque<i64> = VecDeque::new();
    let mut current = 0;
    for i in 0..m {
        que.push_back(a[i]);
        current += (i as i64 + 1) * a[i];
    }
    let cumsum = vec![vec![0], a.iter().cumsum().collect::<Vec<i64>>()].concat();
    let mut ans = current.clone();
    for i in m..n {
        current += m as i64 * a[i] - (cumsum[i] - cumsum[i - m]);
        ans = ans.max(current);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
