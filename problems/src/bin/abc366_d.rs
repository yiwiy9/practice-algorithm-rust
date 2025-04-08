use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[[usize; n]; n]; n],
        q: usize,
        lr: [[Usize1; 6]; q],
    }

    // 3次元累積和
    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                s[i + 1][j + 1][k + 1] =
                    a[i][j][k] + s[i][j + 1][k + 1] + s[i + 1][j][k + 1] + s[i + 1][j + 1][k]
                        - s[i][j][k + 1]
                        - s[i][j + 1][k]
                        - s[i + 1][j][k]
                        + s[i][j][k];
            }
        }
    }

    for lr_i in &lr {
        let (lx, rx, ly, ry, lz, rz) = (lr_i[0], lr_i[1], lr_i[2], lr_i[3], lr_i[4], lr_i[5]);
        let sum =
            s[rx + 1][ry + 1][rz + 1] + s[lx][ly][rz + 1] + s[lx][ry + 1][lz] + s[rx + 1][ly][lz]
                - s[lx][ry + 1][rz + 1]
                - s[rx + 1][ly][rz + 1]
                - s[rx + 1][ry + 1][lz]
                - s[lx][ly][lz];
        println!("{}", sum);
    }
}
