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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,k:usize,p:usize,dev:[[usize;k+1];n]
    }
    let mut mp: HashMap<Vec<usize>, usize> = HashMap::new();
    mp.insert(vec![0; k], 0);
    for i in 0..n {
        let c = dev[i][0];
        let up = &dev[i][1..];
        let mut tmp: HashMap<Vec<usize>, usize> = mp.clone();
        for (v, cost) in &mp {
            let mut ins = vec![0; k];
            for j in 0..k {
                ins[j] = (v[j] + up[j]).min(p);
            }
            if let Some(current) = tmp.get(&ins) {
                if *current > c + cost {
                    tmp.insert(ins, c + cost);
                }
            } else {
                tmp.insert(ins, c + cost);
            }
        }
        mp = tmp;
    }
    let mut ans = 1e18 as usize;
    for (v, cost) in &mp {
        if v.iter().filter(|x| x >= &&p).count() == k {
            ans = ans.min(*cost);
        }
    }
    if ans != 1e18 as usize {
        println!("{}", ans);
        return;
    }
    println!("-1");
}
fn main() {
    solve();
}
