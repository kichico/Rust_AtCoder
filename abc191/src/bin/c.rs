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
        h:usize,w:usize,field:[Chars;h]
    }
    let mut ans = 0;
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];
    for i in 0..h {
        for j in 0..w {
                let mut neighbor = 0;
                for k in 0..4 {
                    let r = i as i64 + dy[k];
                    let c = j as i64 + dx[k];
                    if r < 0
                        || c < 0
                        || r >= h as i64
                        || c >= w as i64
                        || field[r as usize][c as usize] == '.'
                    {
                        neighbor += 1;
                        continue;
                    }
                }
                if neighbor==1||neighbor==3{
                    ans+=1;
                }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
