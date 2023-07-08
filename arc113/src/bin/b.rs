#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use num_traits::Pow;
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
        a:Chars,b:u32,c:u32
    }
    //println!("{:?}", a);
    let last_a_digit = (*a.iter().next_back().unwrap()) as i32 - 48;
    let ans = match last_a_digit {
        1 => 1,
        5 => 5,
        6 => 6,
        0 => 0,
        2 => {
            if b % 4 == 0 {
                6
            } else if b % 4 == 1 {
                2
            } else if b % 4 == 2 {
                if c == 1 {
                    4
                } else {
                    6
                }
            } else {
                if c % 2 == 1 {
                    8
                } else {
                    2
                }
            }
        }
        3 => {
            if b % 4 == 0 {
                1
            } else if b % 4 == 1 {
                3
            } else if b % 4 == 2 {
                if c == 1 {
                    9
                } else {
                    1
                }
            } else {
                if c % 2 == 1 {
                    7
                } else {
                    3
                }
            }
        }
        4 => {
            if b % 2 == 0 {
                6
            } else {
                4
            }
        }
        7 => {
            if b % 4 == 0 {
                1
            } else if b % 4 == 1 {
                7
            } else if b % 4 == 2 {
                if c == 1 {
                    9
                } else {
                    1
                }
            } else {
                if c % 2 == 1 {
                    3
                } else {
                    7
                }
            }
        }
        8 => {
            if b % 4 == 0 {
                6
            } else if b % 4 == 1 {
                8
            } else if b % 4 == 2 {
                if c == 1 {
                    4
                } else {
                    6
                }
            } else {
                if c % 2 == 1 {
                    2
                } else {
                    8
                }
            }
        }
        9 => {
            if b % 4 == 0 {
                1
            } else if b % 2 == 0 {
                1
            } else {
                9
            }
        }
        _ => -1,
    };
    println!("{}", ans);
    //match last_digit {}
}

fn local() {
    let (ak, bk, ck) = (9u32, 5u32, 4u32);
    for aa in 7..=7 {
        let a: BigInt = BigInt::from_u32(aa).unwrap();
        for b in 2..=bk {
            for c in 1..=ck {
                println!("{} {} {} {}", a, b, c, a.pow(b.pow(c)));
            }
        }
    }
}
fn main() {
    //local();
    solve();
}
