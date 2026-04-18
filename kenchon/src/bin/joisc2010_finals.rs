use ac_library::Dsu;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joisc2010/tasks/joisc2010_finals
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   都市集合の連結成分の話にする。今選手がいる都市数は連結成分数として持てばよい。
/// - それについて、何が分かれば答えになる？
///   連結成分数を k 個にするために採用した辺集合の最小コストが分かれば答えになる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   個々の選手の動き方は不要。成分を 1 つ減らす最小コストの辺を順に採ればよく、同一成分内の辺は不要。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   辺をコスト順に sort し、DSU で別成分なら merge してコスト加算する。連結成分数が k になったら止める。
///
/// AC: 25分
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut abc: [(Usize1, Usize1, usize); m],
    }

    abc.sort_by(|a, b| a.2.cmp(&b.2));

    let mut dsu = Dsu::new(n);

    let mut ans: usize = 0;
    let mut components_cnt = n;
    for &(a, b, c) in &abc {
        if components_cnt == k {
            break;
        }
        if dsu.same(a, b) {
            continue;
        }
        ans += c;
        dsu.merge(a, b);
        components_cnt -= 1;
    }

    println!("{}", ans);
}
