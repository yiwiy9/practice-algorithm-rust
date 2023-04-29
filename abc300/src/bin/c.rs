use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let n = h.min(w);

    let mut cross_centers: Vec<(usize, usize)> = vec![];
    for i in 0..h {
        for j in 0..w {
            if 0 < i
                && i < h - 1
                && 0 < j
                && j < w - 1
                && c[i][j] == '#'
                && c[i - 1][j - 1] == '#'
                && c[i - 1][j + 1] == '#'
                && c[i + 1][j - 1] == '#'
                && c[i + 1][j + 1] == '#'
            {
                cross_centers.push((i, j));
            }
        }
    }

    let mut ans = vec![];
    for x in 2..=n + 1 {
        let mut next_cross_centers = vec![];
        for &(i, j) in &cross_centers {
            if x <= i
                && i + x <= h - 1
                && x <= j
                && j + x <= w - 1
                && c[i - x][j - x] == '#'
                && c[i - x][j + x] == '#'
                && c[i + x][j - x] == '#'
                && c[i + x][j + x] == '#'
            {
                next_cross_centers.push((i, j));
            }
        }
        ans.push(cross_centers.len() - next_cross_centers.len());
        cross_centers = next_cross_centers;
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
