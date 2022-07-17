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
        n: usize,a:[String;n]
    }
    let mut v: Vec<Vec<char>> = vec![Vec::new(); n];
    for i in 0..n {
        v[i] = a[i].chars().collect();
    }
    let a = v;
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    let dx = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let dy = vec![0, 1, 1, 1, 0, -1, -1, -1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let mut x = j;
                let mut y = i;
                let mut ss: Vec<char> = Vec::new();
                ss.push(a[y][x]);
                for _m in 0..n - 1 {
                    let mut nx = x.clone() as i64 + dx[k];
                    let mut ny = y.clone() as i64 + dy[k];
                    if nx < 0 {
                        nx += n as i64;
                    } else if nx >= n as i64 {
                        nx -= n as i64;
                    }
                    if ny < 0 {
                        ny += n as i64;
                    } else if ny >= n as i64 {
                        ny -= n as i64;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    ss.push(a[ny][nx]);
                    x = nx;
                    y = ny;
                }
                let num: String = ss.iter().collect();
                let num: i64 = num.parse().unwrap();
                ans.insert(num);
            }
        }
    }
    println!("{}", ans.iter().next_back().unwrap());
}

fn main() {
    solve();
}
