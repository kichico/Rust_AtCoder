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
        n:usize,s:[Chars;n]
    }
    let head: HashSet<char> = HashSet::from(['H', 'D', 'C', 'S']);
    let tail = HashSet::from([
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ]);
    let mut cards: HashSet<Vec<char>> = HashSet::new();
    for card in s {
        if head.contains(&card[0]) && tail.contains(&card[1]) {
            cards.insert(card);
        }
    }
    let ans = if cards.len() == n { "Yes" } else { "No" };
    println!("{}", ans);
}
fn main() {
    solve();
}
