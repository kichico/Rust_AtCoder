#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
        ha:usize,wa:usize,fa:[Chars;ha],hb:usize,wb:usize,fb:[Chars;hb],hx:usize,wx:usize,fx:[Chars;hx]
    }
    let n = 20;
    let base = vec![vec!['.'; n]; n];
    let mut blk_a = 0;
    let mut blk_b = 0;
    let mut blk_x = 0;
    for i in 0..ha {
        for j in 0..wa {
            if fa[i][j] == '#' {
                blk_a += 1;
            }
        }
    }
    for i in 0..hb {
        for j in 0..wb {
            if fb[i][j] == '#' {
                blk_b += 1;
            }
        }
    }
    for i in 0..hx {
        for j in 0..wx {
            if fx[i][j] == '#' {
                blk_x += 1;
            }
        }
    }
    if blk_a.max(blk_b) > blk_x || blk_a + blk_b < blk_x {
        println!("No");
        return;
    }
    'first: for a in 0..n * n {
        let ax = a % n;
        let ay = a / n;
        'second: for b in 0..n * n {
            let mut c = base.clone();
            let bx = b % n;
            let by = b / n;
            for i in 0..ha {
                for j in 0..wa {
                    let row = i + ay;
                    let col = j + ax;
                    if row >= n || col >= n {
                        continue 'first;
                    }
                    c[row][col] = fa[i][j];
                }
            }
            for i in 0..hb {
                for j in 0..wb {
                    let row = i + by;
                    let col = j + bx;
                    if row >= n || col >= n {
                        continue 'second;
                    }
                    c[row][col] = fb[i][j];
                }
            }
            for i in 0..n {
                println!("{}", c[i].iter().join(" "));
            }
            println!("-------------------------------");
            'outer: for i in 0..n {
                'inner: for j in 0..n {
                    for y in 0..hx {
                        for x in 0..wx {
                            let row = i + y;
                            let col = j + x;
                            if row >= n {
                                break 'outer;
                            } else if col >= n {
                                break 'inner;
                            }
                            if c[row][col] != fx[y][x] {
                                continue 'inner;
                            }
                        }
                    }
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
