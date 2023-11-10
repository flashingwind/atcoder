use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    let mut cnt_105=0;
    for i in 1..=n{
        let mut cnt=0;
        if i%2==1{
            for j in 1..=n{
                if i%j==0{
                    cnt+=1;
                }
            }
        }
        if cnt==8{
            cnt_105+=1;
        }
    }
    println!("{}",cnt_105);
}
