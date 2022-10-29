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

#[allow(non_snake_case)]
fn solve() {
    input! {
        mut n:i64
    }
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    let mut nex: Vec<char> = Vec::new();
    ans.insert(n.clone());
    while n > 0 {
        if n % 2 == 0 {
            nex.push('0');
        } else {
            nex.push('1');
        }
        n /= 2;
    }
    nex.reverse();
    let s = nex;
    let mut pos: Vec<usize> = Vec::new();
    for i in 0..s.len() {
        if s[i] == '1' {
            pos.push(i);
        }
    }
    for i in 0..pos.len() {
        let it = pos.iter().combinations(i);
        for v in it {
            let mut nu = 0;
            for p in 0..v.len() {
                nu += 2.pow((s.len() - v[p] - 1) as u32);
            }
            ans.insert(nu);
            //println!("{}", v.into_iter().join(" "));
        }
    }
    for c in ans {
        println!("{}", c);
    }
}

fn main() {
    solve();
}
