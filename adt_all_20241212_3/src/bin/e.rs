use proconio::input;

/// https://atcoder.jp/contests/adt_all_20241212_3/tasks/abc227_c
/// https://atcoder.jp/contests/adt_all_20241212_3/editorial/2906
/// まず愚直に実装する場合の計算量を考えろ
/// 誤差が不安な時は、ループの条件に浮動小数点を使わないようにする
/// => ループ内の先頭でbreakするようにする
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for a in 1..=n {
        if a * a * a > n {
            break;
        }
        for b in a..=n {
            if a * b * b > n {
                break;
            }
            ans += n / a / b - b + 1;
        }
    }

    println!("{}", ans);
}
