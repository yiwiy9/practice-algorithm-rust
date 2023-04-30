use proconio::input;

/**
 * https://imoz.jp/algorithms/imos_method.html
 * 二次元いもす法
 */
fn main() {
    input! {
        n: usize,
        papers: [(usize,usize,usize,usize); n],
    }

    let mut fields = vec![vec![0_i32; 1001]; 1001];
    for &(lx, ly, rx, ry) in &papers {
        fields[ly][lx] += 1;
        fields[ly][rx] -= 1;
        fields[ry][lx] -= 1;
        fields[ry][rx] += 1;
    }

    for y in 0..1001 {
        for x in 1..1001 {
            fields[y][x] += fields[y][x - 1];
        }
    }
    for y in 1..1001 {
        for x in 0..1001 {
            fields[y][x] += fields[y - 1][x];
        }
    }

    let mut ans = vec![0; n + 1];
    for y in 0..1001 {
        for x in 0..1001 {
            ans[fields[y][x] as usize] += 1;
        }
    }

    for s in ans.iter().take(n + 1).skip(1) {
        println!("{}", s);
    }
}
