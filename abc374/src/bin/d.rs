use proconio::input;

fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
        abcd: [(f64,f64,f64,f64); n],
    }

    let mut ans = f64::MAX;
    let mut order = (0..n).collect::<Vec<_>>();

    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        ans = ans.min(rec(n, s, t, order, &abcd, 0, true));
        ans = ans.min(rec(n, s, t, order, &abcd, 0, false));
    });

    println!("{:.10}", ans);
}

fn rec(
    n: usize,
    s: f64,
    t: f64,
    order: &[usize],
    abcd: &[(f64, f64, f64, f64)],
    i: usize,
    is_from_a: bool,
) -> f64 {
    let (from_a, from_b, from_c, from_d) = abcd[order[i]];
    let time_t = if is_from_a {
        let time_t = calc_time(from_a, from_b, from_c, from_d, t);
        time_t
            + if i == 0 {
                calc_time(0.0, 0.0, from_a, from_b, s)
            } else {
                0.0
            }
    } else {
        let time_t = calc_time(from_c, from_d, from_a, from_b, t);
        time_t
            + if i == 0 {
                calc_time(0.0, 0.0, from_c, from_d, s)
            } else {
                0.0
            }
    };

    if i == n - 1 {
        return time_t;
    }

    let (to_a, to_b, to_c, to_d) = abcd[order[i + 1]];

    let rec_from_a = rec(n, s, t, order, abcd, i + 1, true);
    let rec_from_c = rec(n, s, t, order, abcd, i + 1, false);

    let time_s = if is_from_a {
        let to_a = calc_time(from_c, from_d, to_a, to_b, s) + rec_from_a;
        let to_c = calc_time(from_c, from_d, to_c, to_d, s) + rec_from_c;
        to_a.min(to_c)
    } else {
        let to_a = calc_time(from_a, from_b, to_a, to_b, s) + rec_from_a;
        let to_c = calc_time(from_a, from_b, to_c, to_d, s) + rec_from_c;
        to_a.min(to_c)
    };

    time_t + time_s
}

fn calc_time(from_x: f64, from_y: f64, to_x: f64, to_y: f64, d: f64) -> f64 {
    ((from_x - to_x) * (from_x - to_x) + (from_y - to_y) * (from_y - to_y)).sqrt() / d
}
