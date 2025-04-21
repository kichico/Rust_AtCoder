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
        s:Chars
    }
    let mut s = s.iter().collect::<VecDeque<_>>();
    let t = s.pop_front().unwrap();
    if t != &'<' {
        println!("No");
        return;
    }
    let t = s.pop_back().unwrap();
    if t != &'>' {
        println!("No");
        return;
    }
    if s.iter().filter(|x| x != &&&'=').count() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
