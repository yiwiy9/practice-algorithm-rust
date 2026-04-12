use proconio::input;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/abc453/tasks/abc453_e
/// https://atcoder.jp/contests/abc453/editorial/18528
/// - 固定するもの:
///   チーム A の人数 `i` を固定する。するとチーム B の人数は `n - i` に確定する。
///   以後、各選手について「サイズ `i` の A に入れるか」「サイズ `n - i` の B に入れるか」だけを見ればよい。
/// - 欲しい量:
///   固定した `i` に対する有効なチーム分けの個数。
///   各選手を `Aのみ / Bのみ / 両方OK / 不可` に 4 分類すると、
///   `不可` が 1 人でもいれば 0 通り、
///   そうでなければ `Aのみ` を全員 A、`Bのみ` を全員 B に強制配置したあと、
///   `両方OK` の人から A に `i - only_a` 人選ぶだけなので `C(both, i - only_a)` 通り。
/// - 必要クエリ:
///   各選手 `j` について
///   `a_ok[j] = [L_j <= i <= R_j]`,
///   `b_ok[j] = [L_j <= n - i <= R_j]`
///   を知りたい。これが分かれば 4 分類のどこに属するかが一意に決まる。
/// - 単調性 / 境界:
///   `i` を左から右へ sweep すると、各選手の `a_ok / b_ok` が変わるのは
///   `i = L_j, R_j + 1, n - R_j, n - L_j + 1` の高々 4 回だけ。
///   したがって各 `i` で全員を見直す必要はなく、境界で状態が変わる選手だけ再計算すればよい。
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let comb = ModComb::new(n, MOD);

    // `i` を 1 ずつ動かすとき、状態変化が起こる境界だけを事前登録する。
    let mut events = vec![vec![]; n + 2];
    for (idx, &(l, r)) in lr.iter().enumerate() {
        events[l].push(idx);
        events[r + 1].push(idx);
        events[n - r].push(idx);
        events[n - l + 1].push(idx);
    }

    let mut a_ok = vec![0usize; n];
    let mut b_ok = vec![0usize; n];

    // cnt[a][b] で 4 分類の人数を持つ。
    // a=1 なら A に入れる、b=1 なら B に入れる。
    // [1][1]=両方OK, [1][0]=Aのみ, [0][1]=Bのみ, [0][0]=不可。
    let mut cnt = [[0usize; 2]; 2];
    cnt[0][0] = n;

    let mut ans = 0usize;
    for i in 1..n {
        // `i-1 -> i` で状態が変わる可能性がある選手だけ 4 分類を更新する。
        for &idx in &events[i] {
            cnt[a_ok[idx]][b_ok[idx]] -= 1;

            let (l, r) = lr[idx];
            a_ok[idx] = usize::from(l <= i && i <= r);
            b_ok[idx] = usize::from(l <= n - i && n - i <= r);

            cnt[a_ok[idx]][b_ok[idx]] += 1;
        }

        let none = cnt[0][0];
        let only_b = cnt[0][1];
        let only_a = cnt[1][0];
        let both = cnt[1][1];

        // `不可` がいたら全員をどこかに割り当てられない。
        // `Aのみ` / `Bのみ` が定員を超える場合も不可能。
        if none > 0 || only_a > i || only_b > n - i {
            continue;
        }

        // 強制配置を置いたあと、自由に振れるのは `両方OK` だけ。
        // A に不足している `i - only_a` 人をここから選べば、残りは自動的に B に入る。
        ans += comb.combination(both, i - only_a);
        ans %= MOD;
    }

    println!("{}", ans);
}

struct ModComb {
    modulo: usize,
    fac: Vec<usize>,
    ifac: Vec<usize>,
}

impl ModComb {
    fn new(max_n: usize, modulo: usize) -> Self {
        let mut fac = vec![1; max_n + 1];
        let mut ifac = vec![1; max_n + 1];
        let mut inv = vec![1; max_n + 1];

        for i in 2..=max_n {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            ifac[i] = ifac[i - 1] * inv[i] % modulo;
        }

        Self { modulo, fac, ifac }
    }

    fn combination(&self, n: usize, k: usize) -> usize {
        if k > n {
            return 0;
        }
        self.fac[n] * self.ifac[k] % self.modulo * self.ifac[n - k] % self.modulo
    }
}
