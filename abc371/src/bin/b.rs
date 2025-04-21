#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use petgraph::algo::{is_bipartite_undirected, is_isomorphic_subgraph_matching};
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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,baby:[(Usize1,char);m]
    }
    let mut is_chonan = vec![false; n];
    let mut ans = Vec::new();
    for (house, gender) in baby {
        if is_chonan[house] || gender == 'F' {
            ans.push("No");
        } else {
            ans.push("Yes");
            is_chonan[house] = true;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
