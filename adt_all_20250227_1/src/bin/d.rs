use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        xy: [(i64,i64); n],
    }

    let mut min_dist_from_light = vec![INF; n];
    for (i, &(x, y)) in xy.iter().enumerate() {
        for &a_i in &a {
            let (light_x, light_y) = xy[a_i];
            let dist = (x - light_x) * (x - light_x) + (y - light_y) * (y - light_y);
            min_dist_from_light[i] = min_dist_from_light[i].min(dist);
        }
    }

    let min_max = *min_dist_from_light.iter().max().unwrap() as f64;

    println!("{}", min_max.sqrt());
}
