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
use std::usize;

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
fn dfs(
    tree: &Vec<Vec<usize>>,
    ans: &mut Vec<(usize, usize, usize)>,
    now: usize,
    par: usize,
    leaf: &mut usize,
) {
    if tree[now].len() == 1 && tree[now][0] == par {
        ans[now] = (*leaf, *leaf, 0);
        *leaf += 1;
    }
    for e in &tree[now] {
        if *e == par {
            continue;
        }
        dfs(tree, ans, *e, now, leaf);
        ans[now].0 = ans[now].0.min(ans[*e].0);
        ans[now].1 = ans[now].1.max(ans[*e].1);
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,edge:[(Usize1,Usize1);n-1]
    }
    let mut tree = vec![Vec::new(); n];
    for (from, to) in &edge {
        tree[*from].push(*to);
        tree[*to].push(*from);
    }
    let ans = &mut vec![(1e18 as usize, 0, 0); n];
    dfs(&tree, ans, 0, 0, &mut 1);
    for i in 0..n {
        let (l, r, _) = ans[i];
        println!("{} {}", l, r);
    }
}
fn main() {
    solve();
}
