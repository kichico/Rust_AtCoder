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
        n:usize
    }
    let mut field = vec![vec!["0".to_string(); n]; n];
    field[n / 2][n / 2] = "T".to_string();
    let mut r = 0;
    let mut c = 0;
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut idx = 0;
    for i in 0..n * n - 1 {
        field[r][c] = (i + 1).to_string();
        let ri = r as i64;
        let ci = c as i64;
        if (c == n - 1 && (r == 0 || r == n - 1)) || (c == 0 && r == n - 1) {
            idx += 1;
            idx %= 4;
        } else if field[(ri + dir[idx].0) as usize][(ci + dir[idx].1) as usize] != "0".to_string() {
            idx += 1;
            idx %= 4;
        }
        let (nr, nc) = dir[idx];
        r = (ri + nr) as usize;
        c = (ci + nc) as usize;
    }
    for i in 0..n {
        println!("{}", field[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
