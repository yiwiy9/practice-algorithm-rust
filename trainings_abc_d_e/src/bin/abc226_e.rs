use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc226/tasks/abc226_e
 * https://atcoder.jp/contests/abc226/editorial/2889
 * ちゃんと考察しろ
 * => 大雑把に見積もるのではなく、細かく考察するのも大事
 *
 * 1. 連結成分ごとに独立なので、連結成分ごとに考える
 * 2. 各頂点とその頂点から他の頂点へのびる辺は1対1に対応するので、N' = M' (N'>=3)となる
 * 3.ある頂点が存在して、その頂点と他の頂点を結ぶ無向辺がちょうど 1 本であるようなものが存在するとき
 *   => 取り除いても答えに影響しない
 * 4. 3を繰り返すと、以下に帰着できる
 *   => どの頂点についても、その頂点と他の頂点を結ぶ無向辺が 2 本以上存在するとき
 *   => すべての頂点の次数が 2 であるような連結な単純無向グラフであるのでサイクルとなる
 * 5. 答えはそれぞれの連結成分のサイクルの向きが2通りあるので、2^(連結成分数) 通り（ただし、すべての連結成分で N'=M'）
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    for &(u, v) in &uv {
        dsu.merge(u, v);
    }

    let mut edge_cnt = HashMap::new();
    for &(u, _) in &uv {
        let u = dsu.leader(u);
        *edge_cnt.entry(u).or_insert(0) += 1;
    }

    let mut ok = true;
    for v in 0..n {
        if !edge_cnt.contains_key(&dsu.leader(v)) || edge_cnt[&dsu.leader(v)] != dsu.size(v) {
            ok = false;
            break;
        }
    }

    println!(
        "{}",
        if ok {
            mod_pow(2, edge_cnt.len(), MOD)
        } else {
            0
        }
    );
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let base = base % modulo;
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
