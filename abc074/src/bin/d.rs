#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use petgraph::visit;
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
        n:usize,field:[[usize;n];n]
    }
    let mut dist = vec![vec![1e12 as usize; n]; n];
    let mut prev = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            prev[i][j] = i;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > field[i][k] + field[k][j] {
                    dist[i][j] = field[i][k] + field[k][j];
                    prev[i][j] = prev[k][j];
                }
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if dist[i][j] != field[i][j] {
                println!("-1");
                return;
            }
        }
    }
    let mut visited = vec![vec![false; n]; n];
    let mut cost = 0;
    for i in 0..n {
        for j in i..n {
            if i == j {
                continue;
            }
            let from = i.clone();
            let to = j.clone();
            let mut pre = prev[from][to];
            while !visited[pre][to] {
                visited[pre][to] = true;
                cost += dist[pre][to];
                pre = prev[pre][to];
                if pre == from {
                    break;
                }
            }
        }
    }
    println!("{}", cost);
}
fn main() {
    solve();
}
