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
use std::mem::take;
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
        n:usize,mut a:[Usize1;n]
    }
    let mut ans = Vec::new();
    let mut dic = HashMap::new();
    let mut b = a.clone();
    b.sort();
    if a == b {
        println!("0");
        return;
    }
    for i in 0..n {
        dic.insert(a[i].clone(), i);
    }
    for i in 0..n - 1 {
        let pos = *dic.get(&i).unwrap();
        if pos == i {
            continue;
        }
        let other = a[i].clone();
        //let tmp = take(&mut a[pos]);
        a[pos] = other;
        a[i] = i;
        ans.push((pos + 1, i + 1));
        let new_p = dic.get_mut(&other).unwrap();
        *new_p = pos;
    }
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i.min(j), j.max(i));
    }
}
fn main() {
    solve();
}
