#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        k: usize,
        a: [i64;n],
    }
    let mut right: usize = 0;
    let mut st: HashMap<i64, i64> = HashMap::new();
    let mut ans: usize = 0;
    for left in 0..n {
        if left > 0 {
            let pt = st.entry(a[left - 1]).or_insert(0);
            *pt -= 1;
            if st.get(&a[left - 1]) == Some(&0) {
                st.remove(&a[left - 1]);
            }
        }
        if left == right {
            let pt = st.entry(a[left]).or_insert(0);
            *pt += 1;
            right += 1;
        }
        while right < n && (st.len() < k || st.contains_key(&a[right])) {
            let p = st.entry(a[right]).or_insert(0);
            *p += 1;
            right += 1;
        }

        dbg!(&st, right);
        dbg!(left, right);
        ans = max(right - left, ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
