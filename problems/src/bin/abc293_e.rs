use proconio::input;

/// https://atcoder.jp/contests/abc293/tasks/abc293_e
/// https://atcoder.jp/contests/abc293/editorial/5976
/// X <= 10^12 なので 平方分割することを考えます。
/// X^(1/2) <= 10^6 なので、X^(1/2)-1 までの数を全探索して、
/// さらに A^(X^(1/2)) をかけると、A^(X^(1/2)*X^(1/2)-1) まで計算できる。
/// あと残る項は、X - (X^(1/2)*X^(1/2)) で、高々 10^6 なので、
/// X - (X^(1/2)*X^(1/2)) 回、 C *= A, C +=1 で計算できる。
fn main() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    }

    let x_sqrt = (x as f64).sqrt() as usize - 1;

    let mut b = 0;
    for _ in 0..x_sqrt {
        b *= a;
        b %= m;
        b += 1;
        b %= m;
    }

    let mut a_pow = 1;
    for _ in 0..x_sqrt {
        a_pow *= a;
        a_pow %= m;
    }

    let mut c = 0;
    for _ in 0..x_sqrt {
        c *= a_pow;
        c %= m;
        c += b;
        c %= m;
    }

    for _ in 0..(x - x_sqrt * x_sqrt) {
        c *= a;
        c %= m;
        c += 1;
        c %= m;
    }

    println!("{}", c);
}
