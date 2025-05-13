use proconio::input;

const MOD: usize = 998244353;

/// condition:
/// 1. リンゴ   << （バナナ, ブドウ）
/// 2. オレンジ << ブドウ
///
/// concern:
/// 1. バナナの先頭とブドウの先頭がどちらが前かで場合分け？
/// 2. そもそもどうやって数え上げる？
///    - 組み合わせ: i~jまでの範囲でリンゴをa個選び、残りの(j-i+1)-a個からオレンジをb個選ぶ
/// 3. 場合分け
///    - （リンゴ、オレンジ）<<（バナナ、ブドウ）
///    - リンゴ<<（オレンジ、バナナ）<<ブドウ     リンゴ<<オレンジ<<バナナ<<ブドウの1被りあり
///
/// 4. 2 2 2 2 の場合
///    - (r,r,o,o) << (b,b,g,g)
///    - r,r << (o,o,b,b) << g,g
///    - 上記以外で条件を満たすパターン
///      - r,o,r,b,b,o,g,g
///      - o がbの先頭の前に来る個数で全探索すれば良い
///      - これだけでは不十分で、g先頭の前に来るbの個数の全探索も必要　（愚直: O(N^2)）
// fn main() {
//     input! {
//         a: usize,
//         b: usize,
//         c: usize,
//         d: usize,
//     }

//     let comb = ModComb::new(a + b + c + d + 1, MOD);

//     let mut ans = 0;
//     for i in 0..=b {
//         let mut cur = 0;
//         if i == b {
//             cur += comb.combination(c + d, d);
//         } else {
//             for j in 0..c {
//                 cur += comb.combination(b - i + j, j) * comb.combination(c - 1 - j + d - 1, d - 1)
//                     % MOD;
//                 cur %= MOD;
//             }
//         }
//         ans += cur * comb.combination(a + i, i) % MOD;
//         ans %= MOD;
//     }

//     println!("{}", ans);
// }

/// 解答
/// https://atcoder.jp/contests/abc405/tasks/abc405_e
/// https://atcoder.jp/contests/abc405/editorial/13004
///
/// 「A+i 個のリンゴとバナナの列」の隙間にオレンジを差し込む方法
///
/// 隙間に差し込むという発想がなかった
/// → あとはブドウの先頭より左側に来るバナナの個数で全探索すれば良い
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let comb = ModComb::new(a + b + c + d + 1, MOD);

    let mut ans = 0;
    for i in 0..=c {
        let front = comb.combination(a + b + i, b);
        let back = comb.combination(c - i + d - 1, d - 1);
        ans += front * back % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}

pub struct ModComb {
    modulo: usize,
    fac: Vec<usize>,
    finv: Vec<usize>,
}
impl ModComb {
    pub fn new(cap: usize, modulo: usize) -> Self {
        let mut fac = vec![0; cap];
        let mut finv = vec![0; cap];
        let mut inv = vec![0; cap];
        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..cap {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
        Self { modulo, fac, finv }
    }
    pub fn combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
    }
    pub fn homogeneous(&self, n: usize, k: usize) -> usize {
        self.combination(n + k - 1, k)
    }
    pub fn large_n_combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        let mut res = 1;
        for i in (n - k + 1)..=n {
            res = res * i % self.modulo;
        }
        res * self.finv[k] % self.modulo
    }
}
