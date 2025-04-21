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
fn dfs(
    family: &Vec<Vec<usize>>,
    hoken: &mut Vec<i64>,
    now: usize,
    rest: usize,
    complete: &mut Vec<bool>,
) {
    if complete[now] {
        return;
    }
    hoken[now] += 1;
    if rest == 0 {
        if family[now].is_empty() {
            complete[now] = true;
        }
        return;
    }
    let mut cnt = 0;
    for t in &family[now] {
        dfs(family, hoken, *t, rest - 1, complete);
        if complete[*t] {
            cnt += 1;
        }
    }
    if cnt == family[now].len() {
        complete[now] = true;
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,par:[Usize1;n-1],ins:[(Usize1,usize);m]
    }
    let mut family: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut ins_max = vec![0; n];
    for i in 0..m {
        ins_max[ins[i].0] = ins_max[ins[i].0].max(ins[i].1);
    }
    for (i, p) in enumerate(par) {
        family[p].push(i + 1);
    }
    let mut search: Vec<(usize, usize)> = Vec::new();
    for (p, d) in enumerate(ins_max) {
        if d != 0 {
            search.push((p, d));
        }
    }
    search.sort_by(|x, y| (x.1.cmp(&y.1)).reverse().then(x.0.cmp(&y.0)));
    let hoken = &mut vec![0; n];
    let complete = &mut vec![false; n];
    for (p, d) in search {
        dfs(&family, hoken, p, d, complete);
    }
    let ans = hoken.iter().filter(|x| **x >= 1).count();
    println!("{}", ans);
}

fn dp_solve() {
    input! {
        n:usize,m:usize,par:[Usize1;n-1],ins:[(Usize1,usize);m]
    }
    let mut family: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut ins_max = vec![0; n];
    for i in 0..m {
        ins_max[ins[i].0] = ins_max[ins[i].0].max(ins[i].1);
    }
    for (i, p) in enumerate(&par) {
        family[*p].push(i + 1);
    }
    let mut dp = vec![0; n];
    for (p, d) in enumerate(ins_max) {
        if d != 0 {
            dp[p] = d + 1;
        }
    }
    for i in 1..n {
        if dp[par[i - 1]] >= 1 {
            dp[i] = dp[i].max(dp[par[i - 1]] - 1);
        }
    }
    dbg!(&dp);
    println!("{}", dp.iter().filter(|x| **x >= 1).count());
}
fn main() {
    dp_solve();
}
