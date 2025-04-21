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
        h:i64,w:i64,n:usize,point:[(i64,i64);n]
    }
    let mut ans: Vec<BTreeSet<(i64, i64)>> = vec![BTreeSet::new(); 10];
    let blacks: BTreeSet<(i64, i64)> = BTreeSet::from_iter(point.into_iter());
    let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
    for (r, c) in &blacks {
        for i in 0..8 {
            let nr = *r + dy[i];
            let nc = *c + dx[i];
            if nr <= 1 || nc <= 1 || nr >= h || nc >= w {
                continue;
            }
            let black = blacks.range((nr - 1, nc - 1)..(nr + 1, nc + 1)).count();
            ans[black].insert((nr, nc));
        }
    }
    for i in 0..10 {
        println!("{}", ans[i].len());
    }
}
fn main() {
    solve();
}
