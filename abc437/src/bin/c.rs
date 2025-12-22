use proconio::input;

/// https://atcoder.jp/contests/abc437/tasks/abc437_c
/// https://atcoder.jp/contests/abc437/editorial/14862
/// → 最初からうまくやろうとするな、ペンを動かせ
///
/// 試行錯誤の一つとして、まず最初に全てのトナカイを片方の状態に寄せて、
/// そこから条件を満たしながら、徐々にもう片方の状態に移動させていくというアプローチがある
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            wp: [(usize,usize); n],
        }

        let mut wp_sum = wp.iter().map(|&(w, p)| w + p).collect::<Vec<_>>();
        wp_sum.sort();

        // 全トナカイがそりを引く状態からスタートする
        let mut power: usize = wp.iter().map(|&(_, p)| p).sum();
        for (i, &wp) in wp_sum.iter().enumerate() {
            if wp > power {
                println!("{}", i);
                break;
            }

            // 軽くて力がないやつからそりに乗せていく
            power -= wp;
        }
    }
}
