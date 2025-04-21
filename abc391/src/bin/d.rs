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
        n:usize,w:usize,blocks:[(usize,usize);n],q:usize,query:[(usize,Usize1);q]
    }
    let mut remain_blocks: BTreeSet<(usize, usize)> = BTreeSet::new(); //高さ,id
    let mut st: VecDeque<BTreeMap<usize, usize>> = VecDeque::new();
    let mut positions: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for i in 0..n {
        remain_blocks.insert((blocks[i].1, i));
        positions.insert(i, blocks[i]);
    }
    for (h, id) in &remain_blocks {
        let (x, y) = *positions.get(id).unwrap();
        if y == 1 {
            if st.is_empty() {
                st.push_back(BTreeMap::new());
            }
            st[y].insert(x, *id);
        } else if let Some(prev_row) = st.get(y - 1) {
            if let Some(prev_x) = prev_row.get(&x) {}
        }
    }
    let mut dropped = BTreeSet::new();
    let mut last_update = 0;
    for (time, id) in query {
        let spend = time - last_update;
        let mut tmp_remain = BTreeSet::new();
        for (h, box_id) in &remain_blocks {
            if dropped.contains(box_id) {
                continue;
            }
            let (x, mut y) = positions.get(box_id).unwrap();
            y = y.saturating_sub(spend).max(1);
            if !st[*x].is_empty() {
                let top_box = *st[*x].iter().last().unwrap();
                let (_top_x, top_y) = *positions.get(&top_box).unwrap();
                if top_y >= y || top_y + 1 == y {
                    y = top_y.clone() + 1;
                    st[*x].push_back(*box_id);
                }
            }
            tmp_remain.insert((y, box_id));
        }
        let will_drop = st.iter().map(|x| x.len()).max();
    }
}
fn main() {
    solve();
}
