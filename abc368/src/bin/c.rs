#[allow(unused_imports)]
use itertools::*;
use nalgebra::VectorViewMut1;
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
    g: &Vec<Vec<usize>>,
    v: &HashSet<usize>,
    ans: &mut HashSet<usize>,
    now: usize,
    visited: &mut Vec<bool>,
) -> bool {
    if visited[now] {
        return false;
    }
    visited[now] = true;
    if g[now].len() == 0 {
        if v.contains(&now) {
            ans.insert(now);
            return true;
        } else {
            return false;
        }
    }
    let is_ok = g[now].iter().any(|&x| dfs(g, v, ans, x, visited));
    if is_ok || v.contains(&now) {
        ans.insert(now);
        return true;
    }
    return false;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,k:usize,edge:[(Usize1,Usize1);n-1],v:[Usize1;k]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let v: HashSet<usize> = HashSet::from_iter(v.into_iter());
    for (a, b) in &edge {
        g[*a].push(*b);
        g[*b].push(*a);
    }
    let visited = &mut vec![false; n];
    let ans: &mut HashSet<usize> = &mut HashSet::new();
    dfs(&g, &v, ans, 0, visited);
    println!("{}", ans.len());
}
fn main() {
    solve();
}
