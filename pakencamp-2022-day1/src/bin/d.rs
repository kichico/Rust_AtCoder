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
        mut n:usize,k:usize
    }
    let mut ans = vec![0; k];
    ans[0] = 1;
    n -= 1;
    for i in 1..k {
        if n > 2 * (k - i) {
            ans[i] = ans[i - 1] + 2;
            n -= 2;
        } else {
            ans[i] = ans[i - 1] + 1;
            n -= 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
