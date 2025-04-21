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
        s:Chars
    }
    let mut cnt: HashMap<char, usize> = HashMap::new();
    for c in s {
        let e = cnt.entry(c).or_insert(0);
        *e += 1;
    }
    let mut ans: Vec<BTreeSet<char>> = vec![BTreeSet::new(); 1111];
    let mut biggest = 0;
    for (c, num) in cnt {
        ans[num].insert(c);
        biggest = biggest.max(num);
    }
    println!("{}", ans[biggest].first().unwrap());
}
fn main() {
    solve();
}
