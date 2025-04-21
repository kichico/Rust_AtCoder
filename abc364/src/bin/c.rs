use bitvec::vec::Splice;
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
        n:usize,x:usize,y:usize,mut sweet:[usize;n],mut salty:[usize;n]
    }
    sweet.sort();
    sweet.reverse();
    let mut sum = 0;
    let mut cnt = 0;
    for i in 0..n {
        sum += sweet[i];
        cnt += 1;
        if sum > x {
            break;
        }
    }
    salty.sort();
    salty.reverse();
    sum = 0;
    let mut cnt2 = 0;
    for i in 0..n {
        sum += salty[i];
        cnt2 += 1;
        if sum > y {
            break;
        }
    }
    //println!("{} {}", cnt, cnt2);
    println!("{}", cnt.min(cnt2));
}
fn main() {
    solve();
}
