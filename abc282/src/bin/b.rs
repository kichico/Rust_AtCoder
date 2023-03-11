use itertools::iproduct;
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
use std::iter::FromIterator;
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
        n:usize,p:usize,problem:[Chars;n]
    }
    let mut v: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for (u, i) in problem.iter().enumerate() {
        let tmp = i.iter().positions(|x| *x == 'o').collect::<Vec<usize>>();
        v[u] = HashSet::from_iter(tmp.into_iter());
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if v[i].union(&v[j]).into_iter().count() == p {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
