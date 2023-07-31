use proconio::input;
use std::collections::BTreeSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    let mut can_sell = 0;
    for &a_i in &a {
        if set.contains(&a_i) {
            can_sell += 1;
        } else {
            set.insert(a_i);
        }
    }

    let mut deq = VecDeque::new();
    for &v in &set {
        deq.push_back(v);
    }

    let mut ans = 1;
    loop {
        if !deq.is_empty() && *deq.front().unwrap() == ans {
            deq.pop_front();
            ans += 1;
            continue;
        }

        if can_sell >= 2 {
            can_sell -= 2;
            ans += 1;
            continue;
        }

        if deq.is_empty() {
            break;
        } else {
            deq.pop_back();
            can_sell += 1;
        }
    }

    println!("{}", ans - 1);
}
