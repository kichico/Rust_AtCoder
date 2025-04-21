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
        n:usize,s:[Chars;n]
    }
    let mut dics: Vec<HashMap<char, i64>> = vec![HashMap::new(); n];
    for i in 0..n {
        for c in &s[i] {
            *dics[i].entry(*c).or_insert(0) += 1;
        }
    }
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let mut ans: BTreeMap<char, i64> = BTreeMap::new();
    'outer: for a in alpha.chars() {
        let mut cnt = 1e18 as i64;
        for i in 0..n {
            if !dics[i].contains_key(&a) {
                continue 'outer;
            }
            cnt = cnt.min(*dics[i].get(&a).unwrap());
        }
        ans.insert(a, cnt);
    }
    if ans.is_empty() {
        return;
    } else {
        for (k, v) in ans {
            for _i in 0..v {
                print!("{}", k);
            }
        }
        print!("\n");
    }
}
fn main() {
    solve();
}
