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
fn dfs(
    n: usize,
    t: usize,
    teams: &mut Vec<usize>,
    pair: &Vec<(usize, usize)>,
    cnt: &mut i64,
    now: usize,
) {
    if teams.len() == n {
        let mut check: HashSet<usize> = HashSet::from_iter((0..t).into_iter());
        for i in 0..n {
            check.remove(&teams[i]);
        }
        if !check.is_empty() {
            return;
        }
        for (u, v) in pair {
            if teams[*u] == teams[*v] {
                return;
            }
        }
        *cnt += 1;
        return;
    }
    for i in 0..t {
        teams.push(i);
        dfs(n, t, teams, pair, cnt, now + 1);
        teams.pop();
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,t:usize,m:usize,pair:[(Usize1,Usize1);m]
    }
    let pair = &pair;
    let cnt = &mut 0;
    let teams: &mut Vec<usize> = &mut Vec::new();
    dfs(n, t, teams, pair, cnt, 0);
    println!("{}", cnt);
}
fn main() {
    solve();
}
