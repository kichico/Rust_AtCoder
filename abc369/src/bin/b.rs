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
        n:usize,piano:[(usize,char);n]
    }
    let mut hiroudo = 0;
    let mut lp = usize::MAX;
    let mut rp = usize::MAX;
    for i in 0..n {
        let (a, s) = piano[i];
        if lp == usize::MAX && s == 'L' {
            lp = a;
        }
        if rp == usize::MAX && s == 'R' {
            rp = a;
        }
        if lp != usize::MAX && rp != usize::MAX {
            continue;
        }
    }
    for (a, s) in piano {
        if s == 'L' {
            hiroudo += lp.abs_diff(a);
            lp = a;
        } else {
            hiroudo += rp.abs_diff(a);
            rp = a;
        }
    }
    println!("{}", hiroudo);
}
fn main() {
    solve();
}
