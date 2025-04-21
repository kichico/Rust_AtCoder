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
    let mut rippo = BTreeSet::new();
    for i in 1..=1e6 as usize {
        if i.pow(3) > n {
            break;
        }
        rippo.insert(i.pow(3));
    }
    let mut ans = 1;
    'outer: for v in rippo {
        let s = v.to_string().chars().collect::<Vec<char>>();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                continue 'outer;
            }
        }
        ans = ans.max(v);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
