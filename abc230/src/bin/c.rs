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
        n:i64,x:i64,y:i64,p:i64,q:i64,r:i64,s:i64
    }
    let h = (q - p + 1) as usize;
    let w = (s - r + 1) as usize;
    let mut field = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize];
    let lower: i64 = std::cmp::max(p as i64 - x as i64, r as i64 - y as i64);
    let upper: i64 = std::cmp::min(q as i64 - x as i64, s as i64 - y as i64);
    for i in lower..=upper {
        field[(x + i - p) as usize][(y + i - r) as usize] = '#';
    }
    let lower = std::cmp::max(p - x, y - s);
    let upper = std::cmp::min(q - x, y - r);
    for i in lower..=upper {
        field[(x + i - p) as usize][(y - i - r) as usize] = '#';
    }
    for i in 0..h {
        println!("{}", field[i].iter().join(""));
    }
}
fn main() {
    solve();
}
