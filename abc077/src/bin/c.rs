#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut a:[i64;n],mut b:[i64;n],mut c:[i64;n]
    }
    a.sort();
    b.sort();
    c.sort();
    let mut ans = 0;
    let mut b_sum = vec![0; n];
    for i in 0..n {
        b_sum[i] = n - c.upper_bound(&b[i]);
    }
    let b_cumsum = b_sum.iter().cumsum().collect::<Vec<usize>>();
    let mut ans = 0;
    for i in 0..n {
        let p = b.upper_bound(&a[i]);
        if p == n {
            continue;
        }
        if b[p] > a[i] {
            ans += if p == 0 {
                b_cumsum[n - 1]
            } else if p < n {
                b_cumsum[n - 1] - b_cumsum[p - 1]
            } else {
                0
            };
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
