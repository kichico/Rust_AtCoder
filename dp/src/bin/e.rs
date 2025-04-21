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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        N:usize,W:usize,items:[(usize,usize);N]
    }
    let V = items.iter().clone().map(|(_x, y)| *y).sum::<usize>();
    let mut dp = vec![vec![W + 1 as usize; V + 1 as usize + 10]; N];
    dp[0][items[0].1] = items[0].0;
    for i in 0..N {
        dp[i][0] = 0;
    }
    for i in 1..N {
        dp[i] = dp[i - 1].clone();
        for v in 0..V {
            if items[i].1 + v > V {
                break;
            }
            dp[i][v + items[i].1] = dp[i][v + items[i].1].min(dp[i - 1][v] + items[i].0);
        }
    }
    for i in (0..=V).rev() {
        if dp[N - 1][i] <= W {
            println!("{}", i);
            //println!("val:{}", dp[N - 1][i]);
            return;
        }
    }
}
fn main() {
    solve();
}
