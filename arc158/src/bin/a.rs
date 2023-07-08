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
        n:usize
    }
    let mut ans = vec![0; n];
    for t in 0..n {
        input! { mut a:[i64;3] }
        a.sort();
        if (a[1] - a[0]) % 2 != 0 || (a[2] - a[1]) % 2 != 0 {
            ans[t] = -1;
            continue;
        } else if a[0] == a[2] {
            continue;
        }
        let mut dist = a[2] - a[0];
        ans[t] += dist / 4;
        dist = dist / 4 + dist % 4;
        println!("rest:{}", ans[t]);
        ans[t] += dist / 2;
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
