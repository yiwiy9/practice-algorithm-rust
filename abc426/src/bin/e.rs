use proconio::input;

const NUM_ITERATION: usize = 100;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            mut s0: (f64, f64),
            mut g0: (f64, f64),
            mut s1: (f64, f64),
            mut g1: (f64, f64),
        }

        // 移動距離が長い方をs0, g0にする
        let dist0 = ((g0.0 - s0.0).powi(2) + (g0.1 - s0.1).powi(2)).sqrt();
        let dist1 = ((g1.0 - s1.0).powi(2) + (g1.1 - s1.1).powi(2)).sqrt();

        if dist0 < dist1 {
            std::mem::swap(&mut s0, &mut s1);
            std::mem::swap(&mut g0, &mut g1);
        }

        let dist0 = ((g0.0 - s0.0).powi(2) + (g0.1 - s0.1).powi(2)).sqrt();
        let dist1 = ((g1.0 - s1.0).powi(2) + (g1.1 - s1.1).powi(2)).sqrt();

        // s1が終点に到達する時点でのs0の位置
        let ratio = dist1 / dist0;
        let tm = (s0.0 + (g0.0 - s0.0) * ratio, s0.1 + (g0.1 - s0.1) * ratio);

        // 相対位置での線分と原点の距離を計算
        let d1 = dist_segment_origin((s0.0 - s1.0, s0.1 - s1.1), (tm.0 - g1.0, tm.1 - g1.1));

        let d2 = dist_segment_origin((tm.0 - g1.0, tm.1 - g1.1), (g0.0 - g1.0, g0.1 - g1.1));

        println!("{:.10}", d1.min(d2));
    }
}

fn dist_segment_origin(a: (f64, f64), b: (f64, f64)) -> f64 {
    let f = |p: f64| -> f64 {
        let x = a.0 + (b.0 - a.0) * p;
        let y = a.1 + (b.1 - a.1) * p;
        (x * x + y * y).sqrt()
    };

    let mut l = 0.0;
    let mut r = 1.0;

    for _ in 0..NUM_ITERATION {
        let ml = (l * 2.0 + r) / 3.0;
        let mr = (l + r * 2.0) / 3.0;
        if f(ml) < f(mr) {
            r = mr;
        } else {
            l = ml;
        }
    }

    f(l)
}
