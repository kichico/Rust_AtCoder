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
        h:usize,w:usize,n:usize
    }
    let mut field = vec![vec!['.'; w]; h];
    let (mut r, mut c) = (0, 0);
    let dx = vec![0, 1, 0, -1];
    let dy = vec![-1, 0, 1, 0];
    let mut dir = 0i32;
    for _i in 0..n {
        //println!("r:{} c:{} dir:{}", r, c, dir);
        if field[r][c] == '.' {
            field[r][c] = '#';
            dir += 1;
            dir %= 4;
        } else {
            field[r][c] = '.';
            dir -= 1;
            dir = dir.rem_euclid(4);
        }
        let mut nr = r as i32 + dy[dir as usize];
        let mut nc = c as i32 + dx[dir as usize];
        if nr < 0 {
            nr += h as i32;
        } else if nr >= h as i32 {
            nr = 0;
        }
        if nc < 0 {
            nc += w as i32;
        } else if nc >= w as i32 {
            nc = 0;
        }
        r = nr as usize;
        c = nc as usize;
    }
    for i in 0..h {
        println!("{}", field[i].iter().join(""));
    }
}
fn main() {
    solve();
}
