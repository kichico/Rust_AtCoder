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
        n:usize,m:usize,mut hp:i64,border:i64,s:Chars,pos:[(i64,i64);m]
    }
    let mut now = (0, 0);
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    let mut item: HashSet<(i64, i64)> = HashSet::from_iter(pos.into_iter());
    for c in s {
        let mut k = 5;
        if c == 'R' {
            k = 1;
        } else if c == 'U' {
            k = 0;
        } else if c == 'D' {
            k = 2;
        } else {
            k = 3;
        }
        now = (now.0 + dx[k], now.1 + dy[k]);
        hp -= 1;
        if hp < 0 {
            println!("No");
            return;
        }
        if hp <= border && item.contains(&now) {
            hp = border.clone();
            item.remove(&now);
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
