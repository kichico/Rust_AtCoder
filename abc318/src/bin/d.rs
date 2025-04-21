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

fn dfs(
    is_used: &mut Vec<bool>,
    weight: &Vec<Vec<usize>>,
    ans: &mut usize,
    current_sum: &mut usize,
) {
    let mut idx = is_used.len() + 10;
    for i in 0..is_used.len() {
        if !is_used[i] {
            idx = i;
            break;
        }
    }
    if idx == is_used.len() + 10 {
        *ans = *ans.max(current_sum);
        return;
    }
    is_used[idx] = true;
    for next in idx + 1..is_used.len() {
        if !is_used[next] {
            *current_sum += weight[idx][next];
            is_used[next] = true;
            dfs(is_used, weight, ans, current_sum);
            is_used[next] = false;
            *current_sum -= weight[idx][next];
        }
    }
    is_used[idx] = false;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n - 1];
    for i in 0..n - 1 {
        input! { node:[usize; n - i - 1] }
        g[i] = node;
    }
    let mut weight = vec![vec![0; 16]; 16];
    for i in 0..n - 1 {
        for j in i + 1..n {
            weight[i][j] = g[i][j - i - 1];
            weight[j][i] = g[i][j - i - 1];
        }
    }
    let mut is_used = vec![false; n];
    if n % 2 == 1 {
        is_used.push(false);
    }
    let ans = &mut 0;
    let current_sum = &mut 0;
    dfs(&mut is_used, &weight, ans, current_sum);
    println!("{}", ans);
}
fn main() {
    solve();
}
