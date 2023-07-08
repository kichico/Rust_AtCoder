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
        h:usize,w:usize,mut field:[Chars;h]
    }
    for i in 0..h {
        for j in 0..w {
            let c = field[i][j].clone();
            if c == '.' || c == '#' {
                continue;
            }
            let n = c as i32 - 48;
            field[i][j] = '.';
            for dx in -n..=n {
                for dy in -n..=n {
                    let r = i as i32 + dy;
                    let c = j as i32 + dx;
                    if r < 0
                        || r as usize >= h
                        || c < 0
                        || c as usize >= w
                        || dx.abs() + dy.abs() > n
                    {
                        continue;
                    }
                    let r = r as usize;
                    let c = c as usize;
                    if field[r][c] == '#' {
                        field[r][c] = '.';
                    }
                }
            }
        }
    }
    for i in 0..h {
        println!("{}", field[i].iter().join(""));
    }
}
fn main() {
    solve();
}
