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
        h:usize,w:usize,f:[Chars;h]
    }
    let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
    for i in 0..h {
        for j in 0..w {
            'outer: for k in 0..8 {
                let mut r = i.clone() as i64;
                let mut c = j.clone() as i64;
                let mut s: String = "".to_string();
                let mut ans: Vec<(i64, i64)> = Vec::new();
                for p in 0..5 {
                    s.push(f[r as usize][c as usize]);
                    ans.push((r + 1, c + 1));
                    r += dy[k];
                    c += dx[k];
                    if p == 4 {
                        continue;
                    }
                    if r < 0 || c < 0 || r >= h as i64 || c >= w as i64 {
                        continue 'outer;
                    }
                }
                if s == "snuke" {
                    for (r, c) in ans {
                        println!("{} {}", r, c);
                    }
                    return;
                }
            }
        }
    }
}
fn main() {
    solve();
}
