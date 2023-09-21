use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        str_a: [Chars;n],
    };
    let mut a = vec![vec![0; n]; n];
    let mut b = vec![vec![0; n]; n];
    for y in 0..n {
        for x in 0..n {
            a[x][y] = str_a[y][x].to_digit(10).unwrap();
            b[x][y] = a[x][y];
        }
    }

    //top
    // println!("ax0={:?}", a[0..n][0]);
    // println!("a00: {:?}", a[0][0]);
    // println!("a10: {:?}", a[1][0]);
    for x in 1..n {
        b[x][0] = a[x - 1][0];
        // println!("b{x}0 <- a{}0: {}", x - 1, a[x - 1][0]);
    }
    // println!("ax0={:?}", a[0..n][0]);
    // println!("a");
    for x in 0..n {
        // print!("{}", a[x][0]);
    }
    // println!();
    // println!("b ");
    for x in 0..n {
        // print!("{}", b[x][0]);
    }
    // println!();
    //right
    for y in 1..n {
        b[n - 1][y] = a[n - 1][y - 1];
    }
    // println!("a");
    for y in 0..n {
        for x in 0..n {
            // print!("({x},{y})={}", a[x][y]);
        }
        // println!();
    }
    // println!("b ");
    for y in 0..n {
        for x in 0..n {
            // print!("({x},{y})={}", b[x][y]);
        }
        // println!();
    }
    //bottom
    for x in 0..n - 1 {
        b[x][n - 1] = a[x + 1][n - 1];
    }
    // println!("a");
    for y in 0..n {
        for x in 0..n {
            // print!("{}", a[x][y]);
        }
        // println!();
    }
    // println!("b ");
    for y in 0..n {
        for x in 0..n {
            // print!("{}", b[x][y]);
        }
        // println!();
    }
    //left
    for y in 0..n - 1 {
        b[0][y] = a[0][y + 1];
    }
    // println!("a");
    for y in 0..n {
        for x in 0..n {
            // print!("{}", a[x][y]);
        }
        // println!();
    }
    // println!("b ");
    for y in 0..n {
        for x in 0..n {
            print!("{}", b[x][y]);
        }
        println!();
    }
}
