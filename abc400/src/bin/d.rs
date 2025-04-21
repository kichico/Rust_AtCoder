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
        h:usize,w:usize,field:[Chars;h],sr:Usize1,sc:Usize1,gr:Usize1,gc:Usize1
    }
    let mut que = VecDeque::new();
    let mut ans = vec![vec![1e18 as usize; w]; h];
    let dc = vec![1, 0, !0, 0];
    let dr = vec![0, 1, 0, !0];
    que.push_back((sr, sc));
    while let Some((r, c)) = que.pop_front() {
        for k in 0..4 {
            let nr = r.wrapping_add(dr[k]);
            let nc = c.wrapping_add(dc[k]);
            if nr >= h || nc >= w {
                continue;
            }
            if field[nr][nc] == '#' {
                que.push_back((nr, nc));
                let nr = nr.wrapping_add(dr[k]);
                let nc = nc.wrapping_add(dc[k]);
                if nr >= h || nc >= w {
                    continue;
                }
                if field[nr][nc] == '#' {
                    que.push_back((nr, nc));
                }
            } else {
                que.push_front((nr, nc));
            }
        }
    }
}
fn main() {
    solve();
}
