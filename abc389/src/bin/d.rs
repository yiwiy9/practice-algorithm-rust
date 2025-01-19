use num_traits::Pow;
use proconio::input;

fn main() {
    input! {
        r: usize,
    }

    let is_in_circle = |i: usize, j: usize| {
        let i = i as f64;
        let j = j as f64;
        (i + 0.5).pow(2) + (j + 0.5).pow(2) <= (r as f64).pow(2)
    };

    let r_by_sqrt2 = (r as f64 / 2.0_f64.sqrt()).round() as usize;
    let mut ans = (r_by_sqrt2 + (r_by_sqrt2 - 1)).pow(2);

    let mut cur = 0_usize;
    let mut cur_j = r_by_sqrt2;
    for i in r_by_sqrt2..=r {
        for j in (1..cur_j).rev() {
            if is_in_circle(i, j) {
                cur += j;
                cur_j = j;
                break;
            }
        }
    }
    ans += cur * 8;
    ans += (r - r_by_sqrt2) * 4;

    println!("{}", ans);
}
