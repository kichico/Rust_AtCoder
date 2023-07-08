#[allow(unused_imports)]
use itertools::*;
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
        n:usize,m:usize,pic:[[Usize1;n];m]
    }
    let mut tonari: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..m {
        for j in 0..n - 1 {
            let u = pic[i][j];
            let v = pic[i][j + 1];
            tonari[u].insert(v);
            tonari[v].insert(u);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += n - tonari[i].len() - 1;
    }
    println!("{}", ans / 2);
}
fn main() {
    solve();
}
