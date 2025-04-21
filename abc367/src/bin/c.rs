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

fn dfs(r: &Vec<usize>, k: &usize, ans: &mut BTreeSet<Vec<usize>>, v: &mut Vec<usize>) {
    if v.len() == r.len() {
        let current_sum = v.iter().sum::<usize>();
        if current_sum % k != 0 {
            return;
        }
        ans.insert(v.to_vec());
        return;
    }
    let len = v.len().clone();
    for val in 1..=r[len] {
        v.push(val);
        dfs(r, k, ans, v);
        v.pop();
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,k:usize,r:[usize;n]
    }
    let mut v: Vec<usize> = Vec::new();
    let mut ans: BTreeSet<Vec<usize>> = BTreeSet::new();
    dfs(&r, &k, &mut ans, &mut v);
    for a in ans {
        println!("{}", a.iter().join(" "));
    }
}
fn main() {
    solve();
}
