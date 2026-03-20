use proconio::{input, marker::Chars};
use std::{
    collections::{HashMap, VecDeque},
    iter::repeat_n,
};

/// https://atcoder.jp/contests/joi2021yo2/tasks/joi2021_yo2_b
/// https://drken1215.hatenablog.com/entry/2020/12/24/014400
/// 最短経路は BFS （メモ化DFSだとTLEは出ないが、WA）
/// そして、全探索は N! ではなく、3^N (3^13 = 1594323)なので間に合う
///
/// TLEしないようにするためには、HashMapのキーをVec<char>ではなく、3進数をencodeしたusizeにする必要あり
fn main() {
    input! {
        n: usize,
        q: usize,
        s: [Chars; q],
    }

    let mut dist = HashMap::new();
    let mut que = VecDeque::new();

    // 'A','B','C'の個数に応じたソート済み配列を始点にして、多始点BFSを行う
    for a in 0..=n {
        for a_b in a..=n {
            let b = a_b - a;
            let c = n - a_b;
            let t = Vec::from_iter(
                repeat_n('A', a)
                    .chain(repeat_n('B', b))
                    .chain(repeat_n('C', c)),
            );
            let key = encode(&t);
            dist.insert(key, 0_usize);
            que.push_back(key);
        }
    }

    while let Some(key) = que.pop_front() {
        let cur_dist = *dist.get(&key).unwrap();
        let t = decode(key, n);
        for i in 1..t.len() {
            let mut new_t = t[..=i].to_vec();
            new_t.reverse();
            if i + 1 < t.len() {
                new_t.extend_from_slice(&t[i + 1..]);
            }

            let new_key = encode(&new_t);

            if dist.contains_key(&new_key) {
                continue;
            }

            dist.insert(new_key, cur_dist + 1);
            que.push_back(new_key);
        }
    }

    for s_i in &s {
        let key = encode(s_i);
        println!("{}", dist.get(&key).unwrap())
    }
}

fn encode(t: &[char]) -> usize {
    let mut res = 0;
    for &t_i in t {
        res = res * 3 + (t_i as usize - 'A' as usize);
    }
    res
}

fn decode(mut x: usize, n: usize) -> Vec<char> {
    let mut res = vec!['A'; n];
    for i in (0..n).rev() {
        let d = x % 3;
        res[i] = match d {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            _ => unreachable!(),
        };
        x /= 3;
    }
    res
}
