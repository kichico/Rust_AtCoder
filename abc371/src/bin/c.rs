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

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mg:usize,edgeg:[(Usize1,Usize1);mg],mh:usize,edgeh:[(Usize1,Usize1);mh]
    }
    let mut cost = Vec::new();
    for i in 0..n - 1 {
        input! { v:[usize;n-i-1]}
        let mut que: VecDeque<usize> = v.iter().cloned().collect::<_>();
        while que.len() < n {
            que.push_front(0);
        }
        cost.push(que);
    }
    let g: HashSet<(usize, usize)> = HashSet::from_iter(edgeg.into_iter());
    let h: HashSet<(usize, usize)> = HashSet::from_iter(edgeh.into_iter());
    let mut total = usize::MAX;
    for v in (0..n).into_iter().permutations(n) {
        let mut current = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let n1 = v[i].min(v[j]);
                let n2 = v[i].max(v[j]);
                if g.contains(&(i, j)) && h.contains(&(n1, n2)) {
                    continue;
                }
                if !g.contains(&(i, j)) && !h.contains(&(n1, n2)) {
                    continue;
                }
                current += cost[n1][n2];
            }
        }
        total = total.min(current);
    }
    println!("{}", total);
}
fn main() {
    solve();
}
