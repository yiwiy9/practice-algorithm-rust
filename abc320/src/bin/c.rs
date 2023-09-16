use proconio::{input, marker::Chars};
const INF: usize = 1 << 30;

fn main() {
    input! {
        m: usize,
        slot: [Chars; 3],
    }

    let mut ans = INF;
    for num in 0..=9 {
        let mut order = (0..=2).collect::<Vec<_>>();

        // next_permutation()
        permutohedron::heap_recursive(&mut order, |order| {
            let mut cur_reel = 0;
            for t in 0..3 * m {
                if slot[order[cur_reel]][t % m] as i32 - '0' as i32 == num {
                    if cur_reel == 2 {
                        ans = ans.min(t);
                        break;
                    }
                    cur_reel += 1;
                }
            }
        });
    }

    println!("{}", if ans == INF { -1 } else { ans as i32 });
}
