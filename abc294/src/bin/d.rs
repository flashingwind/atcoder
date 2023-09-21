use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut uncalled_head = 0;
    let mut unapperd_head = 0;
    let mut is_called = vec![false; n];
    for _ in 0..q {
        input! {
            eve: u32,
        }
        if eve == 1 {
            // call
            is_called[uncalled_head] = true;
            if uncalled_head < unapperd_head {
                unapperd_head = uncalled_head;
            }
            uncalled_head += 1;
        } else if eve == 2 {
            input! {
                mut x: usize
            }
            // to 0-rigin
            x -= 1;
            is_called[x] = false;
            if unapperd_head == x {
                let mut is_none = true;
                unapperd_head += 1;
                for (i, c) in is_called.iter().enumerate().skip(unapperd_head) {
                    if *c {
                        unapperd_head = i;
                        is_none = false;
                        break;
                    }
                }
                if is_none {
                    unapperd_head = 0;
                }
            }
            // println!("proc {} unapperd_head={}", x + 1, unapperd_head + 1);
        } else if eve == 3 {
            println!("{}", unapperd_head + 1);
        }
        // println!("{:?}", called);
    }
}
