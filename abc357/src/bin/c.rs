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
        n:usize
    }
    if n == 0 {
        println!("#");
        return;
    }
    let level_1 = vec![
        vec!['#', '#', '#'],
        vec!['#', '.', '#'],
        vec!['#', '#', '#'],
    ];
    let h = 3usize.pow(n as u32);
    let mut field = vec![vec!['.'; 3usize.pow(n as u32)]; 3usize.pow(n as u32)];
    for row in 0..h {
        for col in (0..h).step_by(3) {
            for j in 0..3 {
                field[row][col + j] = level_1[row % 3][j];
            }
        }
    }
    let target = (0..h)
        .into_iter()
        .map(|x| 1 + 3 * x)
        .collect::<BTreeSet<usize>>();
    for level in (1..=n).rev() {
        let block_size = 3usize.pow((level - 1) as u32);
        for row in (0..h).step_by(block_size) {
            if !target.contains(&(row / block_size)) {
                continue;
            }
            for col in (0..h).step_by(block_size) {
                if !target.contains(&(col / block_size)) {
                    continue;
                }
                //println!("row:{} col:{}", row, col);
                for i in 0..block_size {
                    for j in 0..block_size {
                        field[row + i][col + j] = '.';
                    }
                }
            }
        }
    }

    for i in 0..h {
        println!("{}", field[i].iter().join(""));
    }
}
fn main() {
    solve();
}
