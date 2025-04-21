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
        s:Chars
    }
    let n = s.len();
    let mut ans = 0;
    'outer: for i in 0..n - 2 {
        let left = s[i];
        for step in 1..=n - 2 {
            if i + step * 2 >= n {
                continue 'outer;
            }
            let center = s[i + step];
            let right = s[i + step * 2];
            if left == 'A' && center == 'B' && right == 'C' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
