use proconio::input;

const INF: usize = 1 << 60;

/// 各 i(1≤i≤N) について min(Ai,P) を足し合わせたものを sum とします。
/// P×K>sum の時は明らかに P 個以上のプロジェクトを作ることは不可能です。
/// 実は、逆に P×K≤sum の時は P 個以上のプロジェクトを作ることが可能であることが証明できます。
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = INF / k; // オーバーフローしないように INF / k にしている
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum = 0;
        for &a_i in &a {
            sum += mid.min(a_i);
        }

        if sum >= k * mid {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
