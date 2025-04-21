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
        n:usize,q:usize,query:[Usize1;q]
    }
    let mut num_to_pos = (0..n).into_iter().collect::<Vec<usize>>();
    let mut pos_to_num = (0..n).into_iter().collect::<Vec<usize>>();
    for x in query {
        let mut pos = num_to_pos[x];
        let mut to_left = false;
        if pos == n - 1 {
            pos -= 1;
            to_left = true;
        } else {
            pos += 1;
        }
        //println!("moved:{}", pos);
        let next_num = pos_to_num[pos];
        num_to_pos[x] = pos;
        pos_to_num[pos] = x;
        pos = if to_left { n - 1 } else { pos - 1 };
        num_to_pos[next_num] = pos;
        pos_to_num[pos] = next_num;
        //println!("{}th: {}", cnt, pos_to_num.iter().map(|x| x + 1).join(" "));
    }
    println!("{}", pos_to_num.iter().map(|x| x + 1).join(" "));
}
fn main() {
    solve();
}
