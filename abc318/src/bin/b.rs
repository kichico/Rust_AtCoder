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
fn to_char(x: usize) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,point:[(usize,usize,usize,usize);n]
    }
    let mut field = vec![vec![0; 101]; 101];
    for (x1, x2, y1, y2) in point {
        for x in x1..x2 {
            for y in y1..y2 {
                field[x][y] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if field[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
fn main() {
    solve();
}
