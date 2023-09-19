use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut abdxdy = vec![vec![None; n]; n];
    for _ in 0..m {
        input! {
            mut a: usize,
            mut b: usize,
            x: i64,
            y: i64,
        }
        a -= 1;
        b -= 1;
        abdxdy[a][b] = Some((x, y));
        abdxdy[b][a] = Some((-x, -y));
    }
    let mut pos = vec![None; n];
    pos[0] = Some((0i64, 0i64));
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        for a in 0..n {
            if let Some((x, y)) = pos[a] {
                for (b, &dxdy) in abdxdy[a].iter().enumerate() {
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
    for p in pos.iter() {
        if let Some((x, y)) = p {
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}
