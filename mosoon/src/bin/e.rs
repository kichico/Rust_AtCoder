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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,l:usize,a:[usize;n],b:[usize;m],bad_pair:[(Usize1,Usize1);l]
    }
    let mut dic: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for (s, h) in bad_pair {
        dic[s].insert(h);
    }
    let hukusai = b
        .iter()
        .enumerate()
        .sorted_by(|x, y| x.1.cmp(y.1))
        .collect::<Vec<(usize, &usize)>>();
    let mut ans = 0;
    for i in 0..n {
        let v = &dic[i];
        let mut id = m - 1;
        while id != 0 && v.contains(&hukusai[id].0) {
            id -= 1;
        }
        if id == 0 && v.contains(&hukusai[id].0) {
            continue;
        }
        ans = ans.max(a[i] + hukusai[id].1);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
