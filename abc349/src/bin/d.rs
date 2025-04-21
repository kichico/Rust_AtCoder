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
        mut l:usize,r:usize
    }
    let two_power = (0..61)
        .into_iter()
        .map(|x| 2.pow(x))
        .collect::<Vec<usize>>();
    let mut ans: Vec<(usize, usize)> = Vec::new();
    while l != r {
        let mut bi_max = 0;
        for (i, bi) in enumerate(&two_power) {
            if l.rem_euclid(*bi) == 0 && l + *bi <= r {
                bi_max = i as u32;
            }
        }
        ans.push((l, l + 2.pow(bi_max)));
        l += 2.pow(bi_max);
    }
    println!("{}", ans.len());
    for (a, b) in ans {
        println!("{} {}", a, b);
    }
}
fn main() {
    solve();
}
