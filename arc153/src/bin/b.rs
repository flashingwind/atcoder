use proconio::input;
use proconio::marker::Chars;

struct Rect {
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Rect {
    fn to_i(&self, x: usize) -> usize {
        self.x_max - (x - self.x_min)
    }
    fn to_j(&self, y: usize) -> usize {
        self.y_max - (y - self.y_min)
    }
    fn to_dest_index(&self, x: usize, y: usize) -> (usize, usize) {
        return (self.to_i(x), self.to_j(y));
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [Chars;h],
    };
    arr.insert(0, Vec::new());
    for a in arr.iter_mut() {
        a.insert(0, '0');
    }
    input! {
        q: usize,
    }
    for _t in 1..=q {
        input! {
            a: usize,
            b: usize,
        }

        let mut rs = vec![
            Rect {
                x_min: 1,
                y_min: 1,
                x_max: a,
                y_max: b,
            },
            Rect {
                x_min: 1,
                y_min: b + 1,
                x_max: a,
                y_max: w,
            },
            Rect {
                x_min: a + 1,
                y_min: 1,
                x_max: h,
                y_max: b,
            },
            Rect {
                x_min: a + 1,
                y_min: b + 1,
                x_max: h,
                y_max: w,
            },
        ];
        for (_ri, r) in rs.iter_mut().enumerate() {
            println!("R={} x={}--{}:", _ri, r.x_min, r.x_max);
            for i in r.x_min..=r.x_max {
                println!("y={}--{}:", i - r.x_min + r.y_min, r.y_max);
                for j in (i - r.x_min) + r.y_min..=r.y_max {
                    let (to_i, to_j) = r.to_dest_index(i, j);

                    print!(
                        "     {},{}:{} <->{},{}: {}",
                        i, j, arr[i][j], to_i, to_j, arr[to_i][to_j]
                    );
                    print!(
                        "({}=={}) && ({}=={})",
                        i as isize - r.x_min as isize,
                        j as isize - (i as isize - r.x_min as isize) - r.y_min as isize,
                        to_i as isize - r.x_min as isize,
                        to_j as isize - (to_i as isize - r.x_min as isize) - r.y_min as isize
                    );

                    if !(i - r.x_min == j - (i - r.x_min + 1) - r.y_min + 1
                        && to_i - r.x_min == to_j - (to_i - r.x_min + 1) - r.y_min + 1
                        && (i > to_i))
                    {
                        let tmp = arr[i][j];
                        arr[i][j] = arr[to_i][to_j];
                        arr[to_i][to_j] = tmp;
                        //is_not_checked[i][j] = false;
                        //is_not_checked[to_i][to_j] = false;
                        print!("*")
                    }
                    println!();
                    /*
                    println!(
                        "     {},{}:{} <->{},{}: {}",
                        i,
                        j,
                        arr[i][j],
                        to_j,
                        to_j,
                        arr[to_j][to_j]
                    );
                    */
                }
            }
        }
        /*
        println!("{}:", _t);
        for ai in arr.iter() {
            println!("{:?}", *ai);
        }
        println!();
        */
    }
    //println!("FIN:");
    for i in 1..=h {
        for j in 1..=w {
            print!("{}", arr[i][j]);
        }
        println!();
    }
}
