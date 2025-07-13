use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char,usize); n],
    }

    let mut ans = vec![];
    let mut cnt = 0;
    for (c, l) in cl {
        cnt += l;
        if cnt > 100 {
            println!("Too Long");
            return;
        }
        ans.extend(std::iter::repeat(c).take(l));
    }

    println!("{}", ans.iter().collect::<String>());
}
