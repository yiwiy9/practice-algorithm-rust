use proconio::input;

fn main() {
    input! {
        n: usize,
        papers: [(usize,usize,usize,usize); n],
    }

    let mut fields = vec![vec![0_i32; 101]; 101];
    for &(a, b, c, d) in &papers {
        fields[c][a] += 1;
        fields[c][b] -= 1;
        fields[d][a] -= 1;
        fields[d][b] += 1;
    }

    for y in 0..101 {
        for x in 1..101 {
            fields[y][x] += fields[y][x - 1];
        }
    }
    for y in 1..101 {
        for x in 0..101 {
            fields[y][x] += fields[y - 1][x];
        }
    }

    let mut ans = 0;
    for y in 0..101 {
        for x in 0..101 {
            if fields[y][x] as usize != 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
