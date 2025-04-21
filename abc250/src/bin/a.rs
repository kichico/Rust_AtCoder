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
        h:i64,w:i64,mut r:i64,mut c:i64
    }
    r -= 1;
    c -= 1;
    let mut ans = 0;
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    for i in 0..4 {
        let nr = r + dy[i];
        let nc = c + dx[i];
        if nr < 0 || nc < 0 || nr >= h || nc >= w {
            continue;
        }
        ans += 1;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
