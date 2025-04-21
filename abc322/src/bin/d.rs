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
fn rot(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ret[i][j] = mat[4 - 1 - j][i];
        }
    }
    return ret;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        mut first:[Chars;4],mut second:[Chars;4],mut third:[Chars;4]
    }
    let mut cnt = 0;
    for i in 0..4 {
        for j in 0..4 {
            if first[i][j] == '#' {
                cnt += 1;
            }

            if second[i][j] == '#' {
                cnt += 1;
            }

            if third[i][j] == '#' {
                cnt += 1;
            }
        }
    }
    if cnt != 16 {
        println!("No");
        return;
    }
    for r1 in 0..4 {
        for r2 in 0..4 {
            for r3 in 0..4 {
                let mut f = first.clone();
                for i in 0..r1 {
                    f = rot(f);
                }
                let mut s = second.clone();
                for i in 0..r2 {
                    s = rot(s);
                }
                let mut t = third.clone();
                for i in 0..r3 {
                    t = rot(t);
                }
                for a1 in 0..7 {
                    for a2 in 0..7 {
                        for b1 in 0..7 {
                            for b2 in 0..7 {
                                for c1 in 0..7 {
                                    'outer: for c2 in 0..7 {
                                        let mut field = vec![vec![0; 10]; 10];
                                        for i in a1..a1 + 4 {
                                            for j in a2..a2 + 4 {
                                                if f[i - a1][j - a2] == '#' {
                                                    field[i][j] += 1;
                                                }
                                            }
                                        }
                                        for i in b1..b1 + 4 {
                                            for j in b2..b2 + 4 {
                                                if s[i - b1][j - b2] == '#' {
                                                    field[i][j] += 1;
                                                }
                                            }
                                        }
                                        for i in c1..c1 + 4 {
                                            for j in c2..c2 + 4 {
                                                if t[i - c1][j - c2] == '#' {
                                                    field[i][j] += 1;
                                                }
                                            }
                                        }
                                        for i in 3..7 {
                                            for j in 3..7 {
                                                if field[i][j] != 1 {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        println!("Yes");
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                //third = rot(third);
            }
            //second = rot(second);
        }
        //first = rot(first);
    }
    println!("No");
}
fn main() {
    solve();
}
