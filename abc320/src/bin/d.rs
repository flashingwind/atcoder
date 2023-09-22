use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut abdxdy = vec![vec![None; n + 1]; n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            x: i64,
            y: i64,
        }
        abdxdy[a][b] = Some((x, y));
        abdxdy[b][a] = Some((-x, -y));
    }
    let mut pos = vec![None; n + 1];
    pos[1] = Some((0i64, 0i64));
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        for a in 1..=n {
            if let Some((x, y)) = pos[a] {
                for (b, &dxdy) in abdxdy[a].iter().enumerate().skip(1) {
                    if let Some((dx, dy)) = dxdy {
                        if pos[b] == None {
                            pos[b] = Some((x + dx, y + dy));
                            is_changed = true;
                        }
                    }
                }
            }
        }
    }
    for p in pos.iter().skip(1) {
        if let Some((x, y)) = p {
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}
