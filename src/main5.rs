use proconio::input;

fn main() {
    input! {
        mut n: usize,
        a: usize,
        b: usize,
    }

    let mut x_sum = 0;
    let mut digit_num = vec![0; 5];

    for mut x in 0..n+1 {
        let x_clone = x.clone();
        digit_num[4] = x / 10000;
        x = x - 10000 * digit_num[4];
        digit_num[3] = x / 1000;
        x = x - 1000 * digit_num[3];
        digit_num[2] = x / 100;
        x = x - 100 * digit_num[2];
        digit_num[1] = x / 10;
        x = x - 10 * digit_num[1];
        digit_num[0] = x;

        // sumの使い方よく分からない...
        // let mut digit_sum = digit_num.sum();

        let mut digit_sum = 0;
        for i in &digit_num {
            digit_sum += i;
        }

        if a <= digit_sum && digit_sum <= b {
            // println!{"true"};
            x_sum += x_clone;
        }
    }

    println!("{}", x_sum);
}