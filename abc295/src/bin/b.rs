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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

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
    let mut bomb = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j].is_ascii_digit() {
                bomb.push((i, j));
            }
        }
    }
    for (i, j) in bomb {
        let range = field[i][j].to_digit(10).unwrap() as i32;
        field[i][j] = '.';
        for dy in -range..=range {
            for dx in -range..=range {
                let ny = i as i32 + dy;
                let nx = j as i32 + dx;
                if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if i.abs_diff(ny) + j.abs_diff(nx) > range as usize {
                    continue;
                }
                if !field[ny][nx].is_ascii_digit() {
                    field[ny][nx] = '.';
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
