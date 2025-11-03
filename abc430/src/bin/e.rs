use itertools::Itertools;
use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc430/tasks/abc430_e
/// https://atcoder.jp/contests/abc430/editorial/14330
/// 本番中、普通にオーバーフローすることも考えずに２進数整数で解こうとしてたのセンス無さすぎ
/// => 制約をちゃんと考えろ。２進数をイコールで結ぶことなど無い。それくらいの制約では配列の O(N) は知れてる
///
/// 文字列の中の特定のパターンを検出する問題に有効な文字列アルゴリズムがいくつか存在する
///   - ローリングハッシュ
///   - Suffix Array
///   - Z-algorithm
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a_chars: Chars,
            b_chars: Chars,
        }

        let a_len = a_chars.len();
        let b_len = b_chars.len();

        let mut aab_chars = a_chars.clone();
        aab_chars.extend_from_slice(&a_chars);
        aab_chars.extend_from_slice(&b_chars);

        let aab_rh = RollingHash::new(&aab_chars.iter().join(""));

        let mut ok = false;
        for i in 0..=a_len * 2 - b_len {
            let a_hash = aab_rh.get(i, i + b_len);
            let b_hash = aab_rh.get(2 * a_len, 2 * a_len + b_len);
            if a_hash == b_hash {
                ok = true;
                println!("{i}");
                break;
            }
        }

        if !ok {
            println!("-1");
        }
    }
}

/// ローリングハッシュ（Rolling Hash）
/// - 基礎: https://koseki2580.github.io/study-docs/docs/Algorithm/rolling-hash/
/// - 実践: https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
///     - ロリハ(RollingHash)のModを (2^61 - 1) にすると安全で爆速になってむちゃくちゃ幸せになれます。
/// - 入力は &str を受け取り、UTF-8 の「バイト列」をハッシュします（速くて一般的）
/// - 乱択 base（毎回ランダム）で狙い撃ちに強い
/// - get(l, r) は半開区間 [l, r)
#[derive(Debug, Clone)]
pub struct RollingHash {
    base: u64,
    pow: Vec<u64>,
    hash: Vec<u64>,
}
impl RollingHash {
    /// メルセンヌ素数 2^61 - 1
    pub const MOD: u64 = (1u64 << 61) - 1;
    /// ランダム（乱択）base で構築（実戦は基本これを使う）
    pub fn new(s: &str) -> Self {
        let base = Self::random_base();
        Self::with_base(s, base)
    }
    /// base を明示して構築（テスト・再現用）
    pub fn with_base(s: &str, base: u64) -> Self {
        assert!((2..Self::MOD).contains(&base), "invalid base");
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut pow = Vec::with_capacity(n + 1);
        let mut hash = Vec::with_capacity(n + 1);
        pow.push(1);
        hash.push(0);
        let mut h = 0u64;
        let mut p = 1u64;
        for &b in bytes {
            h = Self::add_mod(Self::mul_mod(h, base), b as u64);
            p = Self::mul_mod(p, base);
            hash.push(h);
            pow.push(p);
        }
        Self { base, pow, hash }
    }
    /// 部分列のハッシュ（半開区間 [l, r)）
    /// 計算式: hash[r] - hash[l] * base^(r-l) (mod MOD)
    #[inline]
    pub fn get(&self, l: usize, r: usize) -> u64 {
        debug_assert!(l <= r && r <= self.len());
        let t = Self::mul_mod(self.hash[l], self.pow[r - l]);
        let mut x = self.hash[r].wrapping_add(Self::MOD).wrapping_sub(t);
        if x >= Self::MOD {
            x -= Self::MOD;
        }
        x
    }
    /// 元文字列の長さ（バイト長）
    #[inline]
    pub fn len(&self) -> usize {
        self.hash.len() - 1
    }
    /// 元文字列が空かどうか
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// このハッシュで使われた base（デバッグ用）
    #[inline]
    pub fn base(&self) -> u64 {
        self.base
    }
    /// (a + b) mod MOD（a,b < MOD を仮定）
    #[inline]
    fn add_mod(a: u64, b: u64) -> u64 {
        let mut s = a + b;
        if s >= Self::MOD {
            s -= Self::MOD;
        }
        s
    }
    /// (a * b) mod MOD（u128 を使った高速・安全な実装）
    /// 2^61-1 はメルセンヌ素数なので、以下のように 61bit で畳み込むと高速に mod できる:
    ///   x mod MOD = (x_low + x_high) mod MOD  （必要ならもう一回だけ畳み込み）
    #[inline]
    fn mul_mod(a: u64, b: u64) -> u64 {
        Self::reduce((a as u128) * (b as u128))
    }
    /// u128 を 2^61-1 で剰余する（1回の fold で十分）
    #[inline]
    fn reduce(x: u128) -> u64 {
        let low = (x as u64) & Self::MOD;
        let high = (x >> 61) as u64;
        let mut s = low + high;
        if s >= Self::MOD {
            s -= Self::MOD;
        }
        s
    }
    /// 依存なし・軽量な疑似乱数（xorshift）で base を作る。
    /// - 0 や 1 は避ける
    /// - 小さすぎる値も避ける（1<<30 以上）→ 分布が偏りにくい/衝突しづらい
    /// - 乱択（ランダム化）により、「狙い撃ち衝突」をほぼ不可能にする
    fn random_base() -> u64 {
        let mut x = 0x9e3779b97f4a7c15u64;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        let span = Self::MOD - (1u64 << 30);
        let b = (x % span) + (1u64 << 30);
        if b <= 1 {
            2
        } else {
            b
        }
    }
}
