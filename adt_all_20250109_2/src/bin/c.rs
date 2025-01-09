use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut ok = true;
    for (i, (s, t)) in st.iter().enumerate() {
        let mut s_ok = true;
        let mut t_ok = true;
        for (j, (u, v)) in st.iter().enumerate() {
            if i == j {
                continue;
            }
            if s == u || s == v {
                s_ok = false;
            }
            if t == u || t == v {
                t_ok = false;
            }
        }
        if !s_ok && !t_ok {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
