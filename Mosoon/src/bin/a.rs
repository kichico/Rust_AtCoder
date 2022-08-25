use alga::general::Field;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::unionfind;
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]

fn solve() {
    input! {
       n:usize,a:[i64;n],mut b:[i64;n]
    }
    b.sort();
    let b = b;
    let mut eachxor: Vec<BTreeSet<i64>> = vec![BTreeSet::new(); n];
    for i in 0..n {
        for y in &b {
            let x = a[i];
            eachxor[i].insert(x ^ *y);
        }
    }
    let mut cnt = vec![0; n];
    for i in 0..n {
        cnt[i] = a[0] ^ b[i];
    }
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    for x in &cnt {
        let mut c = vec![0; n];
        for i in 0..n {
            c[i] = a[i] ^ *x;
        }
        c.sort();
        let mut flg = true;
        for i in 0..n {
            if b[i] != c[i] {
                flg = false;
                break;
            }
        }
        if flg {
            ans.insert(*x);
        }
    }
    if ans.is_empty() {
        println!("0");
    } else {
        println!("{}", ans.len());
        for x in &ans {
            println!("{}", x);
        }
    }
}

fn main() {
    solve();
}
