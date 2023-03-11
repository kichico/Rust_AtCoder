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
use std::iter::FromIterator;
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
        n:usize,robot:[usize;n],m:usize,mc:[usize;m],x:usize
    }
    let mochi: HashSet<usize> = HashSet::from_iter(mc.into_iter());
    let mut visited = vec![false; x + 1];
    visited[0] = true;
    for i in 0..x {
        if !visited[i] {
            continue;
        }
        for j in 0..n {
            if i + robot[j] > x {
                continue;
            }
            if !mochi.contains(&(i + robot[j])) {
                visited[i + robot[j]] = true;
            }
        }
    }
    let ans = if visited[x] { "Yes" } else { "No" };
    println!("{}", ans);
}
fn main() {
    solve();
}
