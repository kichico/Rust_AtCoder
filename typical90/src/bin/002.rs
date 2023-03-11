use bitset_fixed::*;
use fixedbitset::*;
#[allow(unused_imports)]
use itertools::Itertools;
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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    if n % 2 == 1 {
        return;
    }
    let mut set: BTreeSet<Vec<char>> = BTreeSet::new();
    'outer: for i in 0..2.pow(n as u32) {
        let mut par = vec!['('; n];
        for j in (0..n).rev() {
            if i & (1 << j) != 0 {
                par[j] = ')';
            }
        }
        let mut cnt = 0;
        for j in 0..n {
            if par[j] == '(' {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if cnt < 0 {
                continue 'outer;
            }
        }
        if cnt == 0 {
            set.insert(par);
        }
    }
    for v in set {
        println!("{}", v.iter().join(""));
    }
}
fn main() {
    solve();
}
