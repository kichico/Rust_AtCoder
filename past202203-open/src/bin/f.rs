#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
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
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        h:usize,w:usize,n: usize,a:[[i64;w];h],c:[i64;n]
    }
    for i in 0..h {
        for j in 0..w {
            let state = a[i][j] as usize;
            let dx = [1, 0, -1, 0];
            let dy = [0, 1, 0, -1];
            for k in 0..4 {
                let nx = j as i64 + dx[k];
                let ny = i as i64 + dy[k];
                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if a[ny][nx] != a[i][j] && c[state - 1] == c[a[ny][nx] as usize - 1] {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}

fn main() {
    solve();
}
