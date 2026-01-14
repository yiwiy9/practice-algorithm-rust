use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        a: [[Usize1; w]; h],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let mut mod_pos = vec![vec![]; d];
    for (i, row) in a.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            mod_pos[num % d].push((num / d, i + 1, j + 1));
        }
    }

    mod_pos.iter_mut().for_each(|row| row.sort());

    let mut mod_power_s = vec![vec![]; d];
    for (r, pos) in mod_pos.iter().enumerate() {
        let mut power_s = vec![0; pos.len()];
        let (_, mut prev_i, mut prev_j) = pos[0];
        for k in 1..pos.len() {
            let i = pos[k].1;
            let j = pos[k].2;
            power_s[k] = power_s[k - 1]
                + (prev_i as i32 - i as i32).abs()
                + (prev_j as i32 - j as i32).abs();
            prev_i = i;
            prev_j = j;
        }
        mod_power_s[r] = power_s;
    }

    for &(l, r) in &lr {
        let remain = l % d;
        let l_div = l / d;
        let r_div = r / d;
        println!(
            "{}",
            mod_power_s[remain][r_div] - mod_power_s[remain][l_div]
        );
    }
}
