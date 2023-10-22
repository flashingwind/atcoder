use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        // a: [u32;n],
    };
    let mut map = vec![vec![' '; n]; n];
    map[0][0] = '.';
    for y in 0..n {
        if y != 0 {
            if map[y - 1][0] == '#' {
                map[y][0] = '.';
            } else {
                map[y][0] = '#';
            }
        }
        for x in 1..n {
            if map[y][x - 1] == '#' {
                map[y][x] = '.';
            } else {
                map[y][x] = '#';
            }
        }
        // println!("{:?}", map[y]);
    }
    for y in 0..n {
        for i in y * a..(y + 1) * a {
            for x in 0..n {
                if map[y][x] == '#' {
                    print!("{}", "#".repeat(b));
                } else {
                    print!("{}", ".".repeat(b));
                }
            }
            println!();
        }
    }
}
