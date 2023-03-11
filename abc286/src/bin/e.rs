#[allow(unused_imports)]
use itertools::Itertools;
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
fn sum_info(warshall: &mut Vec<Vec<(i64, i64)>>, i: usize, j: usize, k: usize) {
    warshall[i][j].0 = warshall[i][k].0 + warshall[k][j].0;
    warshall[i][j].1 = warshall[i][k].1 + warshall[k][j].1;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,city:[i64;n],edge:[Chars;n],q:usize,query:[(Usize1,Usize1);q]
    }
    let mut warshall = vec![vec![(1e18 as i64, 0); n]; n];
    for i in 0..n {
        for j in 0..n {
            if edge[i][j] == 'Y' {
                warshall[i][j] = (1, city[j]);
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if warshall[i][j].0 > warshall[i][k].0 + warshall[k][j].0 {
                    sum_info(&mut warshall, i, j, k);
                } else if warshall[i][j].0 == warshall[i][k].0 + warshall[k][j].0
                    && warshall[i][j].1 < warshall[i][k].1 + warshall[k][j].1
                {
                    warshall[i][j].1 = warshall[i][k].1 + warshall[k][j].1;
                }
            }
        }
    }
    for (from, to) in query {
        if warshall[from][to].0 == 1e18 as i64 {
            println!("Impossible");
            continue;
        }
        println!(
            "{} {}",
            warshall[from][to].0,
            city[from] + warshall[from][to].1
        );
    }
}
fn main() {
    solve();
}
