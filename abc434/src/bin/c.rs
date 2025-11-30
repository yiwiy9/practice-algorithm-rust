use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            h: i64,
            tlu: [(i64, i64, i64); n],
        }

        let mut ok = true;
        let mut low = h;
        let mut high = h;
        let mut time = 0;
        for &(t, l, u) in &tlu {
            let diff = t - time;

            let new_low = l.max(low - diff);
            let new_high = u.min(high + diff);

            if new_low > u || new_high < l {
                ok = false;
                break;
            }

            low = new_low;
            high = new_high;
            time = t;
        }

        println!("{}", if ok { "Yes" } else { "No" });
    }
}
