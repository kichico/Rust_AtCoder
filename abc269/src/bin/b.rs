#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        f:[Chars;10]
    }
    let (mut r, mut c) = (0, 0);
    let n = 10;
    let mut flg = false;
    for i in 0..n {
        for j in 0..n {
            if f[i][j] == '#' {
                r = i;
                c = j;
                flg = true;
                break;
            }
        }
        if flg {
            break;
        }
    }
    let mut y = r.clone();
    let mut x = c.clone();
    while y + 1 < n && f[y + 1][c] == '#' {
        y += 1;
    }
    while x + 1 < n && f[r][x + 1] == '#' {
        x += 1;
    }
    println!("{} {}\n{} {}", r + 1, y + 1, c + 1, x + 1);
}

fn main() {
    solve();
}
