use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut a = vec![];
    for _ in 0..n {
        input! {
            l_i: usize,
            a_i: [usize; l_i],
        }
        a.push(a_i);
    }

    let mut cur = vec![1];
    for a_i in a {
        let mut next = vec![];
        for &cur_i in &cur {
            for &a_i_j in &a_i {
                if cur_i > x / a_i_j + x % a_i_j {
                    continue;
                }
                next.push(cur_i * a_i_j);
            }
        }
        cur = next;
    }

    println!("{}", cur.iter().filter(|&&y| y == x).count());
}
