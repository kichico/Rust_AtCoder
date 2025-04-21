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
       s:String
    }
    let dic = vec![
        ("tourist".to_string(), 3858),
        ("ksun48".to_string(), 3679),
        ("Benq".to_string(), 3658),
        ("Um_nik".to_string(), 3648),
        ("apiad".to_string(), 3638),
        ("Stonefeang".to_string(), 3630),
        ("ecnerwala".to_string(), 3613),
        ("mnbvmar".to_string(), 3555),
        ("newbiedmy".to_string(), 3516),
        ("semiexp".to_string(), 3481),
    ];
    for (name, rate) in dic {
        if s == name {
            println!("{}", rate);
            return;
        }
    }
}
fn main() {
    solve();
}
