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

fn dfs(
    lines: &Vec<(i32, i32, i32, i32)>,
    mut ans: f64,
    now: f64,
    id: usize,
    x: i32,
    y: i32,
    s: i32,
    t: i32,
) -> f64 {
    if id == lines.len() {
        ans = ans.min(now);
        return ans;
    }
    let (a, b, c, d) = lines[id];
    let dx = x.abs_diff(a) as f64;
    let dy = y.abs_diff(b) as f64;
    let ans = dfs(
        lines,
        ans,
        now + (dx * dx + dy * dy).sqrt() / (s as f64)
            + ((c - a).pow(2) as f64 + (d - b).pow(2) as f64).sqrt() / (t as f64),
        id + 1,
        c,
        d,
        s,
        t,
    );
    let (c, d, a, b) = lines[id];
    let dx = x.abs_diff(a) as f64;
    let dy = y.abs_diff(b) as f64;
    let ans = dfs(
        lines,
        ans,
        now + (dx * dx + dy * dy).sqrt() / (s as f64)
            + ((c - a).pow(2) as f64 + (d - b).pow(2) as f64).sqrt() / (t as f64),
        id + 1,
        c,
        d,
        s,
        t,
    );
    return ans;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:i32,t:i32,ll:[(i32,i32,i32,i32);n]
    }
    let mut ans = f64::MAX;
    for l in (0..n).into_iter().permutations(n) {
        let lines = l.iter().map(|x| ll[*x]).collect::<Vec<_>>();
        ans = dfs(&lines, ans, 0f64, 0, 0, 0, s, t);
    }
    println!("{:.10}", ans);
}
fn main() {
    solve();
}
