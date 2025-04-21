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
use std::thread::current;

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,c:i64,a:[i64;n]
    }
    let mut right = 0;
    let mut ans = 0;
    for left in 0..n - 1 {
        if left >= right {
            right = left + 1;
        }
        if left[]
        while right + 1 < n && current < current + a[right + 1] {
            current += a[right + 1];
            right += 1;
        }
        if ans<current{
            ans
        }
    }
}
fn main() {
    solve();
}
