use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        yy: i64,
        mut a: [[i64;w];h],
    };
    let d = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];
    for yy2 in 1..=yy {
        for x in 0..h {
            for y in 0..w {

                let mut is_underthesea = false;
                if x==0 || x==h-1 || y==0 || y==w-1 || a[x][y]==-1{
                    is_underthesea = true;
                }
                for (dx, dy) in d.iter() {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if 0 <= nx && nx < h as i64 && 0 <= ny && ny < w as i64 {
                        if a[nx as usize][ny as usize] == -1 {
                            is_underthesea = true;
                            break;
                        }
                    }
                    if is_underthesea {
                        break;
                    }
                }
                if a[x][y] <= yy2 && is_underthesea {
                    a[x][y] = -1;
                    for (dx, dy) in d.iter() {
                        let nx = x as i64 + dx;
                        let ny = y as i64 + dy;
                        if 0 <= nx && nx < h as i64 && 0 <= ny && ny < w as i64 {
                            if a[nx as usize][ny as usize] <= yy2 {
                                a[nx as usize][ny as usize] = -1;
                            }
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for x in 0..h {
            for y in 0..w {
                if a[x][y] != -1 {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
