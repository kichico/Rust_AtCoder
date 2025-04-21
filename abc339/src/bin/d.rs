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
        n:usize,mut field:[Chars;n]
    }
    let inf = 1e18 as usize;
    let mut dist = vec![vec![vec![vec![inf; n]; n]; n]; n];
    let mut ps: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if field[i][j] == 'P' {
                ps.push((i, j));
                field[i][j] = '.';
            }
        }
    }
    let (s1r, s1c) = ps[0];
    let (s2r, s2c) = ps[1];
    let dr = vec![1, 0, -1, 0];
    let dc = vec![0, 1, 0, -1];
    dist[s1r][s1c][s2r][s2c] = 0;
    let mut que: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    que.push_back((s1r, s1c, s2r, s2c));
    let n = n as i32;
    while let Some(pp) = que.pop_front() {
        let (p1r, p1c, p2r, p2c) = pp;
        for i in 0..4 {
            let mut np1r = p1r as i32 + dr[i];
            let mut np1c = p1c as i32 + dc[i];
            let mut np2r = p2r as i32 + dr[i];
            let mut np2c = p2c as i32 + dc[i];
            if np1r < 0
                || np1r >= n
                || np1c < 0
                || np1c >= n
                || field[np1r as usize][np1c as usize] == '#'
            {
                np1r = p1r as i32;
                np1c = p1c as i32;
            }
            if np2r < 0
                || np2r >= n
                || np2c < 0
                || np2c >= n
                || field[np2r as usize][np2c as usize] == '#'
            {
                np2r = p2r as i32;
                np2c = p2c as i32;
            }
            let (np1r, np1c, np2r, np2c) =
                (np1r as usize, np1c as usize, np2r as usize, np2c as usize);
            if np1r == p1r && np1c == p1c && np2r == p2r && np2c == p2c {
                continue;
            }
            if dist[np1r][np1c][np2r][np2c] == inf {
                dist[np1r][np1c][np2r][np2c] = dist[p1r][p1c][p2r][p2c] + 1;
                que.push_back((np1r, np1c, np2r, np2c));
            }
        }
    }
    let n = n as usize;
    let mut ans = inf;
    for i in 0..n {
        for j in 0..n {
            ans = ans.min(dist[i][j][i][j]);
        }
    }
    let out = if ans == inf {
        "-1".to_string()
    } else {
        ans.to_string()
    };
    println!("{}", out);
}
fn main() {
    solve();
}
