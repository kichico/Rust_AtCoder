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
        r:Usize1,c:Usize1
    }
    let mut field = vec![vec![0; 15]; 15];
    let mut now: i32 = 6;
    let mut ran = 3;
    let mut yoko = 6;
    while now >= 0 {
        for i in yoko as usize..(yoko + ran) as usize {
            field[now as usize][i] = 1;
        }
        now -= 2;
        ran += 4;
        yoko -= 2;
    }
    let mut now = 8;
    let mut ran = 3;
    let mut yoko = 6;
    while now < 15 {
        for i in yoko as usize..(yoko + ran) as usize {
            field[now as usize][i] = 1;
        }
        now += 2;
        ran += 4;
        yoko -= 2;
    }
    let mut now = 6;
    let mut ran = 3;
    let mut yoko = 6;
    while now >= 0 {
        for i in yoko as usize..(yoko + ran) as usize {
            field[i][now as usize] = 1;
        }
        now -= 2;
        ran += 4;
        yoko -= 2;
    }
    let mut now = 8;
    let mut ran = 3;
    let mut yoko = 6;
    while now < 15 {
        for i in yoko as usize..(yoko + ran) as usize {
            field[i][now as usize] = 1;
        }
        now += 2;
        ran += 4;
        yoko -= 2;
    }
    field[7][6] = 1;
    field[7][8] = 1;
    field[8][7] = 1;
    field[6][7] = 1;
    let ans = if field[r][c] == 1 { "black" } else { "white" };
    println!("{}", ans);
}

fn main() {
    solve();
}
