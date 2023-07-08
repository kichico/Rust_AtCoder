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

fn dfs(visited: &mut Vec<Vec<bool>>, f: &Vec<Vec<char>>, r: usize, c: usize) -> (i64, i64) {
    visited[r][c] = true;
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    let w = f[0].len().clone() as i64;
    let h = f.len().clone() as i64;
    let mut a = 1;
    let mut b = 0;
    for i in 0..4 {
        let nr = r as i64 + dx[i];
        let nc = c as i64 + dy[i];
        if nr < 0 || nc < 0 || nr >= h || nc >= w {
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if !visited[nr][nc] && f[r][c] != f[nr][nc] {
            let cnt = dfs(visited, &f, nr, nc);
            a += cnt.1;
            b += cnt.0;
        }
    }
    return (a, b);
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        h:usize,w:usize,f:[Chars;h]
    }
    let mut visited = vec![vec![false; w]; h];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if !visited[i][j] {
                let cnt = dfs(&mut visited, &f, i, j);
                ans += cnt.0 * cnt.1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
