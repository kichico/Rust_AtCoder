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
        h:usize,w:usize,limit:i64,field:[Chars;h]
    }
    let mut okashi: Vec<(usize, usize)> = Vec::new();
    let mut start = (0, 0);
    let mut goal = start.clone();

    let mut dist = vec![vec![1e18 as i64; h * w]; h * w];
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                continue;
            }
            if field[i][j] == 'o' {
                okashi.push((i, j));
            }
            if field[i][j] == 'S' {
                start = (i, j);
            }
            if field[i][j] == 'G' {
                goal = (i, j);
            }
            dist[i * w + j][i * w + j] = 0;
        }
    }
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let r = i as i64 + dy[k];
                let c = j as i64 + dx[k];
                if r < 0 || c < 0 || r >= h as i64 || c >= w as i64 || field[i][j] == '#' {
                    continue;
                }
                let r = r as usize;
                let c = c as usize;
                if field[r][c] == '#' {
                    continue;
                }
                dist[i * w + j][r * w + c] = 1;
            }
        }
    }

    for k in 0..h * w {
        for i in 0..h * w {
            for j in 0..h * w {
                dist[i][j] = std::cmp::min(dist[i][k] + dist[k][j], dist[i][j]);
            }
        }
    }
    if dist[start.0 * w + start.1][goal.0 * w + goal.1] > limit {
        println!("-1");
        return;
    }
    let mut ans = 0usize;
    for i in 0..10 {
        for v in okashi.iter().permutations(i + 1) {
            let ll = v.len().clone();
            let mut now = start.clone();
            let mut dd = 0;
            for (r, c) in v {
                dd += dist[now.0 * w + now.1][*r * w + *c];
                now = (*r, *c);
            }
            dd += dist[now.0 * w + now.1][goal.0 * w + goal.1];
            if dd <= limit {
                ans = ans.max(ll);
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
