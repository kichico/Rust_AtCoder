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
#[derive(Debug, Clone)]
pub struct RangeSet {
    st: BTreeSet<(usize, usize)>,
    range_length: usize,
}
impl RangeSet {
    pub fn new() -> Self {
        let st: BTreeSet<(usize, usize)> = BTreeSet::new();
        let range_length = 0;
        return RangeSet { st, range_length };
    }
    pub fn len(&self) -> usize {
        return self.st.len();
    }
    pub fn range_len(&self) -> usize {
        return self.range_length;
    }
    pub fn find(&self, target: &usize) -> Option<(usize, usize)> {
        let mut lower_bound = self
            .st
            .range((Unbounded, Excluded((target + 1, target + 1))));
        if let Some(value) = lower_bound.next_back() {
            if value.1 < *target {
                return None;
            } else {
                return Some(*value);
            }
        } else {
            return None;
        }
    }
    pub fn contains(&self, target: &usize) -> bool {
        if let Some(_value) = self.find(target) {
            return true;
        } else {
            return false;
        }
    }
    pub fn insert(&mut self, mut left: usize, mut right: usize) -> bool {
        if left >= right {
            return false;
        }
        let mut existing = self.st.range((Unbounded, Included((right, right))));
        let mut will_remove: Vec<(usize, usize)> = Vec::new();
        let mut range_len_diff = 0i64;
        while let Some((el, er)) = existing.next_back() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
                left = left.min(*el);
                right = right.max(*er);
            } else {
                break;
            }
        }
        let mut existing = self.st.range((Included((left, left)), Unbounded));
        while let Some((el, er)) = existing.next() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
                left = left.min(*el);
                right = right.max(*er);
            } else {
                break;
            }
        }
        if !will_remove.is_empty() {
            for p in will_remove {
                self.st.remove(&p);
                range_len_diff -= (p.1 - p.0) as i64;
            }
        }
        self.st.insert((left, right));
        range_len_diff += (right - left) as i64;
        self.range_length = (self.range_length as i64 + range_len_diff) as usize;
        return true;
    }
    pub fn remove(&mut self, left: usize, right: usize) {
        assert!(left <= right);
        let mut existing = self.st.range((Unbounded, Included((right, right))));
        let mut will_remove: Vec<(usize, usize)> = Vec::new();
        let mut range_len_diff = 0i64;
        while let Some((el, er)) = existing.next_back() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
            } else {
                break;
            }
        }
        if !will_remove.is_empty() {
            for p in will_remove {
                let (el, er) = p;
                self.st.remove(&p);
                range_len_diff -= (p.1 - p.0) as i64;
                if el <= left && right <= er {
                    self.insert(el, left);
                    self.insert(right, er);
                } else if left <= er && right <= er {
                    self.insert(el, left);
                } else if left <= el && right <= er {
                    self.insert(right, er);
                }
            }
        }
        self.range_length = (self.range_length as i64 + range_len_diff) as usize;
    }
    pub fn mex(&self) -> usize {
        if let Some((_l, r)) = self.find(&0) {
            return r;
        } else {
            return 0;
        }
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,dots:[(Usize1,Usize1);m]
    }
    let mut tate = vec![RangeSet::new(); n];
    let mut yoko = vec![RangeSet::new(); n];

    let mut field = vec![vec![0; n]; n];
    for (r, c) in &dots {
        tate[*c].insert(*r, *r + 1);
        yoko[*r].insert(*c, *c + 1);
        field[*r][*c] = 1;
    }
    let mut row_que: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut col_cnt: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for c in 0..n {
        row_que.insert((m - yoko[c].range_len(), c));
    }
    for c in 0..n {
        col_cnt
            .entry(tate[c].range_len())
            .or_insert(Vec::new())
            .push(c);
    }
    let mut ans: Vec<(usize, usize)> = Vec::new();
    while let Some((mut size, r)) = row_que.pop_first() {
        if size == 0 {
            continue;
        }
        let last_idx = col_cnt.iter().last().unwrap().0.clone();
        let v = col_cnt.get_mut(&last_idx).unwrap();
        let c = v.pop().unwrap();
        field[r][c] = 1;
        ans.push((r + 1, c + 1));
        if v.is_empty() {
            col_cnt.remove(&last_idx);
        }
        if last_idx > 1 {
            col_cnt.entry(last_idx - 1).or_insert(Vec::new()).push(c);
        }
        size -= 1;
        if size == 0 {
            continue;
        }
        row_que.insert((size, r));
    }
    for r in 0..n {
        println!("{}", field[r].iter().join(" "));
    }
}
fn main() {
    solve();
}
