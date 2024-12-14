use proconio::input;

/**
 * https://atcoder.jp/contests/abc238/tasks/abc238_d
 * https://atcoder.jp/contests/abc238/editorial/3359
 * https://atcoder.jp/contests/abc238/editorial/3512
 * https://atcoder.jp/contests/abc238/editorial/3374
 *
 * 和をbit演算に置き換える
 * x + y = (両方が立っているビット * 2) + どちらかが立っているビット
 *       = 2(x&y) + (x^y)
 *
 * x&y = a, x+y = s なので、x^y = s - 2a
 * また、(x&y) & (x^y) = 0 なので、(s - 2a) & a = 0
 *
 * 【おまけ】
 * ビット演算全般について、それぞれ「ビットごとに ○○ する演算」と言い換えられます。
 * ○○ の部分は、例えば
 *
 * AND：min、mod2 での積
 * OR ：max
 * XOR：mod2 での和
 * NOT：mod2 での+1
 *
 * となります。
 */
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a: usize,
            s: usize,
        }

        if s >= 2 * a && (s - 2 * a) & a == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
