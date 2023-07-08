#[allow(unused_imports)]
use itertools::*;
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
use std::iter::FromIterator;
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
        n:usize,q:usize
    }
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut not_connected: HashSet<usize> = HashSet::from_iter((0..n).into_iter());
    for _i in 0..q {
        input! { x:i64 }
        if x == 1 {
            input! { u:Usize1,v:Usize1 }
            g[u].insert(v);
            g[v].insert(u);
            not_connected.remove(&u);
            not_connected.remove(&v);
        } else {
            input! { u:Usize1 }
            for v in g[u].clone() {
                g[v].remove(&u);
                if g[v].len() == 0 {
                    not_connected.insert(v);
                }
            }
            g[u].clear();
            not_connected.insert(u);
        }
        println!("{}", not_connected.len());
    }
}
fn main() {
    solve();
}
