use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a_compressed = vec![];
    let mut cur = a[0];
    let mut cnt = 1;
    for i in 1..n {
        if cur == a[i] {
            a_compressed.push(cnt);
            cnt = 0;
        }
        cur = a[i];
        cnt += 1;
    }
    a_compressed.push(cnt);

    if a_compressed.len() <= 3 {
        println!("{}", n);
        return;
    }

    let mut ans = 0;
    for i in 0..a_compressed.len() - 2 {
        ans = ans.max(a_compressed[i] + a_compressed[i + 1] + a_compressed[i + 2]);
    }

    println!("{}", ans);
}
