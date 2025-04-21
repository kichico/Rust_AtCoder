#[allow(unused_imports)]
use itertools::Itertools;
use maplit::hashset;
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
use std::iter::FromIterator;
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
fn solve() {
    input! {
        n: usize,slot:[Chars;n]
    }
    let mut ans = 100000;
    let mut dic: Vec<Vec<usize>> = vec![Vec::new(); 10];
    for i in 0..n {
        for idx in 0..10 {
            dic[(slot[i][idx] as u32 - 48) as usize].push(idx);
        }
    }
    for i in 0..10 {
        dic[i].sort();
        let mut num_time: BTreeSet<usize> = BTreeSet::new();
        for idx in &dic[i] {
            let mut ix = idx.clone();
            while num_time.contains(&ix) {
                ix += 10;
            }
            num_time.insert(ix);
        }
        ans = ans.min(*num_time.iter().next_back().unwrap());
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
