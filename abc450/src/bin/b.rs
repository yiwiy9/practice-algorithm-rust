use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut c = vec![];
    for i in 1..n {
        input! {
            d_i: [usize; n-i],
        }
        let mut c_i = vec![0; n];
        let mut d_idx = 0;
        for j in i..n {
            c_i[j] = d_i[d_idx];
            d_idx += 1;
        }
        c.push(c_i);
    }

    let mut ok = false;
    for x in 0..n - 2 {
        for z in x + 2..n {
            for y in x + 1..z {
                if c[x][z] > c[x][y] + c[y][z] {
                    ok = true;
                }
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
