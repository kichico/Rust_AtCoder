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
fn rot(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; mat[0].len()]; mat.len()];
    for i in 0..mat[0].len() {
        for j in 0..mat.len() {
            ret[i][j] = mat[mat.len() - 1 - j][i];
        }
    }
    return ret;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut ans = vec![vec!['.'; n]; n];
    let tgt: BTreeSet<usize> = (0..n).into_iter().step_by(2usize).collect();
    for _i in 0..4 {
        for r in 0..n {
            ans[r][0] = '#';
            ans[r][n - 1] = '#';
            if tgt.contains(&r) {
                for c in r..n - r {
                    ans[r][c] = '#';
                }
            }
        }
        ans = rot(ans);
    }

    for i in 0..n {
        println!("{}", ans[i].iter().join(""));
    }
}
fn main() {
    solve();
}
