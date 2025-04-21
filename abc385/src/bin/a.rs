#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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
        nums:[usize;3]
    }
    if nums[0] == nums[1] && nums[1] == nums[2] {
        println!("Yes");
        return;
    }
    for v in (0..=2).combinations(2) {
        let mut st = btreeset! {1,2,0};
        let g1 = v.iter().map(|i| nums[*i]).sum::<usize>();
        for i in v {
            st.remove(&i);
        }
        let g2 = st.iter().map(|i| nums[*i]).sum::<usize>();
        if g1 == g2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
