//! Largest prime factor
//! Problem 3
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

extern crate core;
extern crate num;

use self::core::ops::Shr;
use self::num::BigUint;
use self::num::bigint::ToBigUint;

/// Maximo Comun Divisor usando el algoritmo de Euclides
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut remainder: usize;

    while b != 0 {
        remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

/// Prueba de primalidad de Miller-Rabin
fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 { return true; }
    if n < 2 || n % 2 == 0 { return false; }
    
    fn witnesses(n: usize) -> &'static[usize] {
        match n {
            1 ... 1373652                   => { static A: &'static[usize] = &[2, 3]; A },
            1373652 ... 4759123140          => { static A: &'static[usize] = &[2, 7, 61]; A },
            4759123141 ... 2152302898746    => { static A: &'static[usize] = &[2, 3, 5, 7, 11]; A },
            2152302898747 ... 3474749660382 => { static A: &'static[usize] = &[2, 3, 5, 7, 11, 13]; A },
            _                               => { static A: &'static[usize] = &[2, 325, 9375, 28178, 450775, 9780504, 1795265022]; A }
        }
    }

    let big_n = n.to_biguint().unwrap();
    let big_1 = 1.to_biguint().unwrap();

    let s = (n - 1).trailing_zeros();
    let d = (n - 1).shr(s);

    for a in witnesses(n) {
        if a >= &n { break; }

        let mut x = num::pow::<BigUint>(a.to_biguint().unwrap(), d) % &big_n;
        if x == big_1 || x == &big_n - &big_1 { continue; }

        let mut t = s;
        while x != &big_n - &big_1 {
            t -= 1;
            if t <= 0 { return false; }
            x = (&x * &x) % &big_n;
            if x == big_1 { return false; }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd() {
        assert_eq!(4, super::gcd(8, 12));
        assert_eq!(6, super::gcd(54, 24));
        assert_eq!(14, super::gcd(42, 56));
        assert_eq!(1, super::gcd(9, 28));
    }

    fn assert_is_prime(n: usize) {
        assert!(super::is_prime(n));
    }

    #[test]
    fn first_primes_below_1000() {
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
                 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
                 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
                 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443,
                 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577,
                 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
                 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
                 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983,
                 991, 997],
            (1usize..1000).filter(|&x| super::is_prime(x)).collect::<Vec<_>>());
    }

    #[test]
    fn large_prime() {
        assert_is_prime(600851475143);
    }
}