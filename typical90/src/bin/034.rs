use proconio::input;
use std::collections::BTreeMap;
// use std::collections::VecDeque;

/**
 * https://twitter.com/e869120/status/1390798852299448322/photo/1
 * 単調性を利用した尺取り法
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();

    let mut ans = 0;
    let mut cr = 0;
    let mut cnt = 0;
    for (i, a_i) in a.iter().enumerate() {
        while cr < n {
            if !map.contains_key(&a[cr]) && cnt == k {
                break;
            }
            if !map.contains_key(&a[cr]) {
                cnt += 1;
            }
            map.entry(a[cr]).and_modify(|cur| *cur += 1).or_insert(1);
            cr += 1;
        }
        ans = ans.max(cr - i);
        map.entry(*a_i).and_modify(|cur| *cur -= 1);
        if map[a_i] == 0 {
            map.remove(a_i);
            cnt -= 1;
        }
    }

    println!("{}", ans);
}

// fn main() {
//     input! {
//         n: usize,
//         k: usize,
//         a: [usize; n],
//     }

//     let mut map = BTreeMap::new();
//     let mut deque = VecDeque::new();

//     let mut ans = 0;
//     for &a_i in &a {
//         map.entry(a_i).and_modify(|cur| *cur += 1).or_insert(1);
//         deque.push_back(a_i);

//         while map.len() > k {
//             let pop = deque.pop_front().unwrap();
//             map.entry(pop).and_modify(|cur| *cur -= 1);
//             if map[&pop] == 0 {
//                 map.remove(&pop);
//             }
//         }

//         ans = ans.max(deque.len());
//     }

//     println!("{}", ans);
// }
