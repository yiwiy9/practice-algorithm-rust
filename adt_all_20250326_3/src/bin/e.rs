use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        d: [usize; n],
    }

    let mut d_r = vec![0; n];
    for i in 0..n {
        d_r[i] = d[i] % (a + b);
    }

    d_r.sort();

    // 範囲をずらして全てが休日に収まるか見るんじゃなくて、
    // それぞれの日の間に平日が入るところがあるかを見る
    let mut ok = false;
    for i in 1..n {
        if d_r[i] - d_r[i - 1] > b {
            ok = true;
            break;
        }
    }
    if d_r[0] + a + b - d_r[n - 1] > b {
        ok = true;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
