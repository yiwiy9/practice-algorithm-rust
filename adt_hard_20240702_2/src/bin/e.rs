use proconio::input;
use std::collections::{BTreeSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sold = 0;
    let mut set = BTreeSet::new();
    for &x in &a {
        if set.contains(&x) {
            sold += 1;
        } else {
            set.insert(x);
        }
    }

    let mut deque = VecDeque::from(set.into_iter().collect::<Vec<_>>());
    let mut ans = 0;
    while deque.len() > 0 {
        if let Some(&x) = deque.front() {
            if x == ans + 1 {
                deque.pop_front();
                ans += 1;
                continue;
            }
        }

        while sold < 2 {
            if deque.pop_back().is_some() {
                sold += 1;
            } else {
                break;
            }
        }

        if sold < 2 {
            break;
        }

        ans += 1;
        sold -= 2;
    }

    ans += sold / 2;

    println!("{}", ans);
}
