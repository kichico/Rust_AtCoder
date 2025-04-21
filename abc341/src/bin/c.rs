use bstr::B;
#[allow(unused_imports)]
use itertools::*;
use maplit::btreemap;
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
use std::collections::btree_map;
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
        h:usize,w:usize,n:usize,t:Chars,field:[Chars;h]
    }
    let mut ans = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '.' {
                ans.push((i, j));
            }
        }
    }
    let dir = btreemap! {'L'=>0,'R'=>1,'U'=>2,'D'=>3};
    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    for c in t {
        let k = *dir.get(&c).unwrap();
        let mut next = Vec::new();
        for (i, j) in &ans {
            let ny = i.clone() as i32 + dy[k];
            let nx = j.clone() as i32 + dx[k];
            if ny < 0
                || nx < 0
                || nx >= w as i32
                || ny >= h as i32
                || field[ny as usize][nx as usize] == '#'
            {
                continue;
            }
            next.push((ny as usize, nx as usize));
        }
        ans = next;
    }
    println!("{}", ans.len());
}
fn main() {
    solve();
}
