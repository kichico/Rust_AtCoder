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
        s:String
    }
    let mut dic: HashMap<String, i64> = HashMap::new();
    dic.insert("Monday".to_string(), 5);
    dic.insert("Tuesday".to_string(), 4);
    dic.insert("Wednesday".to_string(), 3);
    dic.insert("Thursday".to_string(), 2);
    dic.insert("Friday".to_string(), 1);
    println!("{}", dic.get(&s).unwrap());
}

fn main() {
    solve();
}
