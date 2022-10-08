use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }

    // パケット法: ユニークな数を求める
    let d_max = 101;
    let mut num = vec![0; d_max];
    
    for i in d {
        num[i] = 1;
    }

    let mut sum = 0;
    for i in &num {
        sum += i;
    }

    println!("{}", sum);

}