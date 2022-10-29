#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
fn dfs(par: &Vec<i64>, when: &mut Vec<i64>, now: usize) -> i64 {
    if when[now] != -1 {
        return when[now] + 1;
    } else if now == 0 {
        return when[now] + 1;
    }
    let next_now = par[now] as usize;
    return dfs(par, when, next_now);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[i64;n]
    }
    let mut parents = vec![-1; 2 * n + 1];
    for i in 0..n {
        parents[2 * i + 1] = a[i] - 1;
        parents[2 * i + 2] = a[i] - 1;
    }
    println!("{}", parents.iter().join(" "));
    let mut when = vec![-1; 2 * n + 1];
    when[0] = 0;
    for i in 1..2 * n + 1 {
        when[i] = dfs(&parents, &mut when, i);
    }
    for f in when {
        println!("{}", f);
    }
}

fn main() {
    solve();
}
