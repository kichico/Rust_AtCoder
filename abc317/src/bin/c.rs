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
        n:usize,m:usize,road:[(Usize1,Usize1,i64);m]
    }
    let mut ans = 0;
    let mut g: Vec<Vec<i64>> = vec![vec![-1; n]; n];
    for (u, v, cost) in road {
        g[u][v] = cost;
        g[v][u] = cost;
    }
    let it = (0..n).into_iter().permutations(n);
    'outer: for v in it {
        let mut current = 0;
        for i in 1..n {
            if g[v[i - 1]][v[i]] == -1 {
                ans = ans.max(current);
                continue 'outer;
            }
            current += g[v[i - 1]][v[i]];
        }
        ans = ans.max(current);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
