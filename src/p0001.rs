//! Multiples of 3 and 5
//! Problem 1
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

pub fn imperative(below: u64) -> u64 {
    let mut sum = 0;

    for x in 3..below {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    sum
}

pub fn declarative(below: u64) -> u64 {
    (3..below)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn imperative() {
        assert_eq!(233168, super::imperative(1000));
    }

    #[test]
    fn declarative() {
        assert_eq!(233168, super::declarative(1000));
    }
}