use alga::general::AbstractField;
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
        n:i64,m:usize,king:[(i64,i64);m]
    }
    let mut available: BTreeSet<(i64, i64)> = BTreeSet::new();
    let dx = vec![2, 1, -1, -2, -2, -1, 1, 2];
    let dy = vec![1, 2, 2, 1, -1, -2, -2, -1];
    for (y, x) in king {
        available.insert((y, x));
        for k in 0..8 {
            let nx = x + dx[k];
            let ny = y + dy[k];
            if nx > n || ny > n || nx < 1 || ny < 1 {
                continue;
            }
            available.insert((ny, nx));
        }
    }
    println!("{}", n * n - available.len() as i64);
}
fn main() {
    solve();
}
