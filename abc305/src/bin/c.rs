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
    let mut cnt = vec![vec![0; w]; h];
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let r = i as i64 + dy[k];
                let c = j as i64 + dx[k];
                if r < 0 || c < 0 || r >= h as i64 || c >= w as i64 {
                    continue;
                }
                let r = r as usize;
                let c = c as usize;
                if f[r][c] == '#' {
                    cnt[i][j] += 1;
                }
            }
        }
    }
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..h {
        for j in 0..w {
            if f[i][j] == '#' {
                continue;
            }
            if ans < cnt[i][j] {
                ans = cnt[i][j];
                x = j;
                y = i;
            }
        }
    }
    println!("{} {}", y + 1, x + 1);
}
fn main() {
    solve();
}
