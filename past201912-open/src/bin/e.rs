use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,

    };
    let mut ff = vec![vec![false; n + 1]; n + 1];
    for _ in 0..q {
        input! {
            inst:u32
        };
        if inst == 1 {
            //フォロー: 1 a b: ユーザ a がユーザ b (a!=b) をフォロー
            input! {
                a:usize,
                b:usize,
            };
            ff[a][b] = true;
        } else if inst == 2 {
            //フォロー全返し: 2 a: ユーザ a が、その時点でユーザ a をフォローしているユーザ全員をフォロー。
            input! {
                a:usize,
            };
            for (b, b_is_following) in ff.to_owned().iter_mut().enumerate().skip(1) {
                if b_is_following[a] {
                    ff[a][b] = true;
                    // println!("2: {a}→{b}");
                }
            }
            // printff(&ff);
        } else if inst == 3 {
            //フォローフォロー: 3 a: ユーザ a がフォローしている各ユーザ x に対し、ユーザ a が次を行う:
            //    「ユーザ x がフォローしているすべてのユーザ (ユーザ a 自身を除く) をフォローする」。
            input! {
                a:usize,
            };
            for (x, a_is_following_x) in ff[a].to_owned().iter().enumerate().skip(1) {
                if *a_is_following_x {
                    for (b, x_is_following_b) in ff[x].to_owned().iter().enumerate().skip(1) {
                        if *x_is_following_b {
                            ff[a][b] = true;
                            // println!("3: {a}→{b}");
                        }
                    }
                }
            }
            // printff(&ff);
        }
    }
    printff(&ff);
}
fn printff(ff: &Vec<Vec<bool>>) {
    for f in ff.iter().skip(1) {
        for f2 in f.iter().skip(1) {
            if *f2 {
                print!("Y");
            } else {
                print!("N");
            }
        }
        println!();
    }
}
