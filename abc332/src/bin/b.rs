use proconio::input;

fn main() {
    input! {
        k: usize,
        g_0: usize,
        m_0: usize,
    }

    let mut g = 0;
    let mut m = 0;

    for _ in 0..k {
        if g == g_0 {
            g = 0;
        } else if m == 0 {
            m = m_0;
        } else if m >= g_0 - g {
            m -= g_0 - g;
            g = g_0;
        } else {
            g += m;
            m = 0;
        }
    }

    println!("{} {}", g, m);
}
