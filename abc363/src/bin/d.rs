use bitvec::ptr::read;
#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
use std::sync::Arc;
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
        n:i64
    }
    if n <= 10 {
        println!("{}", n - 1);
        return;
    }
    let mut freedom: Vec<i64> = vec![0, 10, 10, 90];
    for i in 4..=37 {
        freedom.push(freedom[i - 2] * 9);
    }
    let total = freedom.iter().cumsum().collect::<Vec<i64>>();
    let keta = total.lower_bound(&n);
    let mut ans: Vec<char> = vec!['0'; keta];
    let mut now = keta - 2;
    let mut sum = 0;
    for k in 0..=keta / 2 {
        for i in 1..10 {
            let v = total[now] + freedom[now].max(1) * i;
            if v + sum > n {
                ans[k] = to_char(i - 1);
                ans[keta - k - 1] = to_char(i - 1);
                sum += total[now] + freedom[now].max(1) * (i - 1);
                break;
            }
        }
        if now == 0 {
            break;
        }
        now -= 1;
    }
    println!("{}", ans.iter().join(""));
}
fn main() {
    solve();
}
