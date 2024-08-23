use proconio::{input, marker::Chars};
const TRANSITION: [[usize; 2]; 3] = [[1, 2], [2, 0], [0, 1]];

/**
 * https://atcoder.jp/contests/abc242/tasks/abc242_d
 * https://atcoder.jp/contests/abc242/editorial/3520
 *
 * 再帰でやるともっとシンプルに書ける
 * k=1のときの、transition_char()でtのデカさを帳消しにするのが肝
 */
fn main() {
    input! {
        s: Chars,
        q: usize,
    }

    let transition_char = |c: char, times: usize| -> char {
        (b'A' + (((c as u8 - b'A') as usize + times) % 3) as u8) as char
    };

    for _ in 0..q {
        input! {
            mut t: usize,
            mut k: usize,
        }

        let mut operations = vec![];
        while t > 0 {
            if k == 1 {
                break;
            }
            operations.push((k + 1) % 2);
            k = (k + 1) / 2;
            t -= 1;
        }

        let mut cur = match transition_char(s[k - 1], t) {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };

        for &op in operations.iter().rev() {
            let next = TRANSITION[cur][op];
            cur = next;
        }

        println!(
            "{}",
            match cur {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                _ => unreachable!(),
            }
        );
    }
}
