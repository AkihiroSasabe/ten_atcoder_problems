use proconio::input;

// 型を調べる
// fn print_typename<T>(_: T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main(){
    // 注意: u32ではなくusizeにしないと、要素にアクセスするとき面倒
    // vect[i as usize]で毎回as usizeしないといけなくなる
    input! {
        n: usize,
        mut vect: [usize; n]
    }

    let mut continue_flag: bool = true;
    let mut count: i32 = -1;

    while continue_flag {
        count += 1;
        for i in 0..n {
            if vect[i] % 2 != 0 {
                continue_flag = false;
                break
            }
            vect[i] = vect[i] / 2;
        }
    }
    println!("{}", count);
}