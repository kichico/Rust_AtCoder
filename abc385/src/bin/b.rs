use amplify::confinement::Collection;
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
        h:usize,w:usize,sr:Usize1,sc:Usize1,field:[Chars;h],t:Chars
    }
    let mut houses = BTreeSet::new();
    let dir = btreemap! {'U'=>0usize,'D'=>1,'L'=>2,'R'=>3};
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '@' {
                houses.push((i, j));
            }
        }
    }
    let dx = vec![0, 0, !0, 1];
    let dy = vec![!0, 1, 0, 0];
    let mut r = sr;
    let mut c = sc;
    let mut ans = 0;
    for C in t {
        let k = *dir.get(&C).unwrap();
        let nr = r.wrapping_add(dy[k]);
        let nc = c.wrapping_add(dx[k]);
        if nr >= h || nc >= w {
            continue;
        }
        if field[nr][nc] == '#' {
            continue;
        }
        if houses.contains(&(nr, nc)) {
            ans += 1;
            houses.remove(&(nr, nc));
        }
        r = nr;
        c = nc;
        //   println!("r={} c={}", r + 1, c + 1);
    }
    println!("{} {} {}", r + 1, c + 1, ans);
}
fn main() {
    solve();
}
