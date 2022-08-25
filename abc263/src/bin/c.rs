#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn dfs(s: &mut Vec<usize>, n: usize, m: usize) {
    let c = *s.iter().next_back().unwrap();
    for i in c + 1..=m {
        if s.len() + 1 == n {
            for p in 0..n - 1 {
                print!("{} ", s[p]);
            }
            println!("{}", i);
            continue;
        } else {
            s.push(i);
            dfs(s, n, m);
        }
    }
    s.pop();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize
    }
    let s: &mut Vec<usize> = &mut Vec::new();
    if n == 1 {
        for i in 1..=m {
            println!("{}", i);
        }
        return;
    }
    for i in 1..=m {
        s.push(i);
        dfs(s, n, m);
    }
}

fn main() {
    solve();
}
