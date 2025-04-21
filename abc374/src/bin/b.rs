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
        mut s:Chars,mut t:Chars
    }
    if s.iter().collect::<String>() == t.iter().collect::<String>() {
        println!("0");
        return;
    }
    let maxi = s.len().max(t.len());
    while s.len() != maxi {
        s.push('!');
    }
    while t.len() != maxi {
        t.push('!');
    }
    for (i, (ss, tt)) in enumerate(s.iter().zip(t)) {
        if ss != &tt {
            println!("{}", i + 1);
            return;
        }
    }
}
fn main() {
    solve();
}
