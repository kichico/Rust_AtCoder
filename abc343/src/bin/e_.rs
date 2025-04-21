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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        v1:i32,v2:i32,v3:i32
    }
    let mut cubic = vec![vec![vec![0; 3]; 2]; 3];
    for j in 0..3 {
        cubic[0][0][j] = 0;
        cubic[0][1][j] = 7;
    }
    for a2 in 0..=7 {
        for b2 in 0..=7 {
            for c2 in 0..=7 {
                for a3 in 0..=7 {
                    for b3 in 0..=7 {
                        for c3 in 0..=7 {
                            let mut cnt = HashMap::new();
                            if b2 == 6 && a3 == 6 {
                                println!("Now");
                            }
                            cnt.insert(1, 0);
                            cnt.insert(2, 0);
                            cnt.insert(3, 0);
                            cubic[1][0][0] = a2;
                            cubic[1][1][0] = a2 + 7;
                            cubic[1][0][1] = b2;
                            cubic[1][1][0] = b2 + 7;
                            cubic[1][0][2] = c2;
                            cubic[1][1][2] = c2 + 7;
                            cubic[2][0][0] = a3;
                            cubic[2][1][0] = a3 + 7;
                            cubic[2][0][1] = b3;
                            cubic[2][1][0] = b3 + 7;
                            cubic[2][0][2] = c3;
                            cubic[2][1][2] = c3 + 7;
                            let mut area = vec![vec![vec![0; 22]; 22]; 22];
                            for i in 0..3 {
                                let [x,y,z]=*cubic[i][0].as_slice() else {todo!()};
                                let [X,Y,Z]=*cubic[i][1].as_slice() else {todo!()};
                                area[z][y][x] += 1;
                                area[z][Y][x] += -1;
                                area[z][y][X] += -1;
                                area[z][Y][X] += 1;
                                area[Z][y][x] += -1;
                                area[Z][Y][x] += 1;
                                area[Z][y][X] += 1;
                                area[Z][Y][X] += -1;
                            }
                            for i in 0..14 {
                                for j in 0..14 {
                                    for k in 1..14 {
                                        area[i][j][k] += area[i][j][k - 1];
                                    }
                                }
                            }
                            for i in 0..14 {
                                for j in 1..14 {
                                    for k in 0..14 {
                                        area[i][j][k] += area[i][j - 1][k];
                                    }
                                }
                            }
                            for i in 1..14 {
                                for j in 0..14 {
                                    for k in 0..14 {
                                        area[i][j][k] += area[i - 1][j][k];
                                    }
                                }
                            }
                            for i in 0..14 {
                                for j in 0..14 {
                                    for k in 0..14 {
                                        *cnt.entry(area[i][j][k]).or_insert(0) += 1;
                                    }
                                }
                            }
                            if cnt.get(&1).unwrap() == &v1
                                && cnt.get(&2).unwrap() == &v2
                                && cnt.get(&3).unwrap() == &v3
                            {
                                println!(
                                    "Yes\n{} {} {} {} {} {} {} {} {}",
                                    0, 0, 0, a2, b2, c2, a3, b3, c3
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
