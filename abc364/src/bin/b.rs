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
        h:usize,w:usize,mut sy:Usize1,mut sx:Usize1,field:[Chars;h],x:Chars
    }

    let dx = vec![1usize, 0, !0, 0];
    let dy = vec![0usize, 1, 0, !0];
    let converter = btreemap! {'L'=>2,'R'=>0,'U'=>3,'D'=>1};
    for c in x {
        let id = *converter.get(&c).unwrap();
        let ny = sy.wrapping_add(dy[id]);
        let nx = sx.wrapping_add(dx[id]);
        if nx >= w || ny >= h {
            continue;
        } else if field[ny][nx] == '#' {
            continue;
        }
        sy = ny;
        sx = nx;
    }
    println!("{} {}", sy + 1, sx + 1);
}
fn main() {
    solve();
}
