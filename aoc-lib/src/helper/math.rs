use num::Integer;

pub fn gcd<T: Integer + Copy>(mut a: T, mut b: T) -> T {
    while a > T::zero() && b > T::zero() {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }

    if a == T::zero() {
        b
    } else {
        a
    }
}

pub fn lcm<T: Integer + Copy>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}
