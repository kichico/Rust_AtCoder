#[allow(unused_imports)]
use itertools::*;
use maplit::btreemap;
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
        mut sa:Chars,mut sb:Chars,mut sc:Chars
    }
    sa.reverse();
    sb.reverse();
    sc.reverse();
    let mut now = 0;
    let dic = btreemap! {'a'=>0usize,'b'=>1,'c'=>2};
    let mut cards = vec![sa, sb, sc];
    while let Some(p) = cards[now].pop() {
        now = *dic.get(&p).unwrap();
    }
    let ans = if now == 0 {
        'A'
    } else if now == 1 {
        'B'
    } else {
        'C'
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
