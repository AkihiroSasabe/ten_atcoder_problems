use proconio::input;

fn main() {
    input! {
        n: u32,
        sum: u32,
    }

    let mut x: i32 = -1;
    let mut y: i32 = -1;
    let mut z: i32 = -1;

    let mut end_flag = false;

    for i in 0..n+1 {
        if end_flag {
            break;
        }
        for j in 0..n+1-i {
            if i * 10000 + j * 5000 + (n-i-j) * 1000 == sum {
                x = i as i32;
                y = j as i32;
                z = (n-i-j) as i32;
                end_flag = true;
                break;
            }           
        }
    }


    println!("{} {} {}", x, y, z);

}