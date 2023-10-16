use proconio::input;

fn main() {
    input! {
        num_p: usize,
        mut probs: [u32;num_p],
        num_d: usize,
        mut drinks: [(usize,u32);num_d],
    };
    probs.insert(0, 0);
    drinks.insert(0, (0, 0));
    for (pid_of_d, new_t) in drinks.iter().skip(1) {
        let mut sum = 0;
        for (pid, t) in probs.iter().enumerate().skip(1) {
            if *pid_of_d == pid {
                sum += *new_t;
                // println!("*pid_of_d == pid={pid} new_t={new_t}");
            } else {
                sum += *t;
                // println!("*pid_of_d != pid={pid} t={t}");
            }
        }
        println!("{}", sum);
    }
}
