use proconio::input;

fn main() {
    input! {
        n: usize,
        num_q: usize,
    };
    let mut b = vec![vec![0usize; 0]; n + 1];
    let mut b_trans = vec![vec![0usize; 0]; 2 * 100000 + 1];
    for _ in 0..num_q {
        input! {
            op: u32,
        };
        if op == 1 {
            input! {
                card_i: usize,
                box_i: usize,
            };
            b[box_i].push(card_i);
            if !b_trans[card_i].contains(&box_i) {
                b_trans[card_i].push(box_i);
            }
        } else if op == 2 {
            input! {
                box_i: usize,
            };
            b[box_i].sort();
            println!(
                "{}",
                b[box_i]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        } else if op == 3 {
            input! {
                card_i: usize,
            };
            b_trans[card_i].sort();
            println!(
                "{}",
                b_trans[card_i]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
