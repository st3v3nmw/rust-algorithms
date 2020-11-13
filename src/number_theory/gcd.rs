// Compute the GCD of m & n using Euclid's Algorithm
fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        return m.abs();
    }
    return gcd(n, m % n);
}

// Compute the GCD of a list
fn gcd_arr(v: Vec<i32>) -> i32 {
    let mut gcd_r = v[0];
    for num in v {
        gcd_r = gcd(gcd_r, num);
    }
    return gcd_r;
}

fn main() {
    println!("{}", gcd(0, 0)); // 0
    println!("{}", gcd(7, 0)); // 7
    println!("{}", gcd(225, 144)); // 9
    println!("{}", gcd(144, -225)); // 9
    println!("{}", gcd(-144, 225)); // 9
    println!("{}", gcd(-144, -225)); // 9

    let v = vec![200, 500, 6000];
    println!("{}", gcd_arr(v)); // 100
}