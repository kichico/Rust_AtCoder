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
        n:usize,h:usize,w:usize
    }
    let mut field = vec![vec!['.'; n * w]; n * h];
    let mut isBlack = false;
    for r in (0..n * h).step_by(h) {
        for c in (0..n * w).step_by(w) {
            let dot = if isBlack { '#' } else { '.' };
            isBlack = !isBlack;
            for y in 0..h {
                for x in 0..w {
                    field[r + y][c + x] = dot;
                }
            }
        }
        if w * n % 2 == 0 {
            isBlack = !isBlack;
        }
    }
    for i in 0..h * n {
        println!("{}", field[i].iter().join(""));
    }
}
fn main() {
    solve();
}
