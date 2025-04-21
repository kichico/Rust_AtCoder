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
        n:usize,m:usize,k:usize
    }
    let mut ans = 0usize;
    let mut res = Vec::new();
    for _i in 0..m {
        input! {c:usize,a:[Usize1;c],r:char}
        res.push((a, r));
    }
    'outer: for bit in 0..(1 << n) {
        let mut standing = vec![false; n];
        for i in 0..n {
            if bit & (1 << i) != 0 {
                standing[i] = true;
            }
        }
        for i in 0..m {
            let (a, r) = &res[i];
            let cnt = a.iter().filter(|x| standing[**x]).count();
            if (cnt >= k && *r == 'x') || (cnt < k && *r == 'o') {
                continue 'outer;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
