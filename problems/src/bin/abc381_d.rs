use proconio::input;
use std::{cmp::Ordering, collections::HashSet, collections::VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut compressed = vec![];
    let mut prev = a[0];
    let mut count = 1;
    for &a_i in a.iter().skip(1) {
        if prev == a_i {
            count += 1;
        } else {
            compressed.push((prev, count));
            prev = a_i;
            count = 1;
        }
    }
    compressed.push((prev, count));

    let mut ans = 0;
    let mut cur = 0;
    let mut used_deq = VecDeque::new();
    let mut used_set = HashSet::new();
    for &(a_i, count) in &compressed {
        match count.cmp(&2) {
            Ordering::Equal => {
                if used_set.contains(&a_i) {
                    while let Some(front) = used_deq.pop_front() {
                        used_set.remove(&front);
                        cur -= 1;
                        if front == a_i {
                            break;
                        }
                    }
                }
                used_deq.push_back(a_i);
                used_set.insert(a_i);
                cur += 1;
                ans = ans.max(cur);
            }
            Ordering::Greater => {
                if used_set.contains(&a_i) {
                    while let Some(front) = used_deq.pop_front() {
                        used_set.remove(&front);
                        cur -= 1;
                        if front == a_i {
                            break;
                        }
                    }
                }
                cur += 1;
                ans = ans.max(cur);

                used_deq = VecDeque::new();
                used_deq.push_back(a_i);
                used_set = HashSet::new();
                used_set.insert(a_i);
                cur = 1;
            }
            Ordering::Less => {
                used_deq = VecDeque::new();
                used_set = HashSet::new();
                cur = 0;
            }
        }
    }
    println!("{}", ans * 2);
}
