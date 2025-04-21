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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        field:[Chars;8]
    }
    let mut rook = Vec::new();
    let n = 8;
    for i in 0..n {
        for j in 0..n {
            if field[i][j] == '#' {
                rook.push((i, j));
            }
        }
    }
    let mut cnt = vec![vec![0; n]; n];
    let dx = vec![1, 0, !0, 0];
    let dy = vec![0, 1, 0, !0];
    for (i, j) in rook {
        cnt[i][j] += 1;
        for k in 0..4 {
            let mut nx = j.clone();
            let mut ny = i.clone();
            loop {
                ny = ny.wrapping_add(dy[k]);
                nx = nx.wrapping_add(dx[k]);
                //println!("ny:{} nx:{}", ny, nx);
                if ny >= n || nx >= n {
                    //  println!("---------");
                    break;
                }
                cnt[ny][nx] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if cnt[i][j] == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
