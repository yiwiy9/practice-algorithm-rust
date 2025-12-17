use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    let mut base = 0;
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            let mut set = BTreeSet::new();
            set.insert(a[i][j]);
            set.insert(a[i + 1][j]);
            set.insert(a[i][j + 1]);
            set.insert(a[i + 1][j + 1]);
            if set.len() >= 3 {
                base += 1;
            }
        }
    }

    let calc_around_points = |i: usize, j: usize, num: usize| -> usize {
        let mut point = 0;

        if i > 0 && j > 0 {
            let mut set = BTreeSet::new();
            set.insert(num);
            set.insert(a[i - 1][j - 1]);
            set.insert(a[i - 1][j]);
            set.insert(a[i][j - 1]);
            if set.len() >= 3 {
                point += 1;
            }
        }

        if i > 0 && j < n - 1 {
            let mut set = BTreeSet::new();
            set.insert(num);
            set.insert(a[i - 1][j]);
            set.insert(a[i - 1][j + 1]);
            set.insert(a[i][j + 1]);
            if set.len() >= 3 {
                point += 1;
            }
        }

        if i < n - 1 && j < n - 1 {
            let mut set = BTreeSet::new();
            set.insert(num);
            set.insert(a[i][j + 1]);
            set.insert(a[i + 1][j + 1]);
            set.insert(a[i + 1][j]);
            if set.len() >= 3 {
                point += 1;
            }
        }

        if i < n - 1 && j > 0 {
            let mut set = BTreeSet::new();
            set.insert(num);
            set.insert(a[i + 1][j]);
            set.insert(a[i + 1][j - 1]);
            set.insert(a[i][j - 1]);
            if set.len() >= 3 {
                point += 1;
            }
        }

        point
    };

    let mut target_vec = (1..=8.min(k)).collect_vec();
    if k > 8 {
        target_vec.push(1 << 60);
    }

    let mut ans = base;
    for i in 0..n {
        for j in 0..n {
            let original = calc_around_points(i, j, a[i][j]);
            for &target_num in &target_vec {
                let target_point = calc_around_points(i, j, target_num);
                if target_point > original {
                    ans = ans.max(base + target_point - original);
                }
            }
        }
    }

    println!("{}", ans);
}
