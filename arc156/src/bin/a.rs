#[allow(unused_imports)]
use itertools::*;
use maplit::hashset;
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
        t:usize
    }
    let mut ans = vec![-1; t];
    for i in 0..t {
        input! { n:usize,s:Chars }
        let cnt = s.iter().filter(|x| **x == '1').count();
        if cnt % 2 != 0 {
            continue;
        }
        let dist = s
            .iter()
            .enumerate()
            .filter(|&x| *x.1 == '1')
            .collect::<Vec<(_, _)>>();
        let s: String = s.iter().collect();
        if n - cnt == n {
            ans[i] = 0;
        } else if n == 3 {
            if s == "101" {
                ans[i] = 1;
            }
        } else if n == 4 {
            if s == "0110" {
                ans[i] = 3;
                continue;
            }
            let two = hashset! {"0101".to_string(),"1010".to_string(),"1001".to_string()};
            if two.contains(&s) {
                ans[i] = 1;
            } else {
                ans[i] = 2;
            }
        } else if cnt == 2 {
            if dist[1].0 - dist[0].0 >= 2 {
                ans[i] = 1;
            } else {
                ans[i] = 2;
            }
        } else {
            ans[i] = cnt as i64 / 2;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
