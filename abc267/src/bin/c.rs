#[allow(unused_imports)]
use itertools::Itertools;
use itertools_num::ItertoolsNum;
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

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:i64,a:[i64;n]
    }
    let mut current = 0;
    for i in 0..m {
        current += a[i as usize] * (i + 1);
    }
    let mut cs: VecDeque<i64> = a.iter().cumsum().collect();
    cs.push_front(0);
    let mut ans = current.clone();
    for i in m as usize..n {
        current -= cs[i as usize] - cs[i - m as usize];
        current += a[i as usize] * m;
        ans = ans.max(current);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
