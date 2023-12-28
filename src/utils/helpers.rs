pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

#[cfg(test)]
pub mod test {
    #[test]
    pub fn test_custom_factorial() {
        for n in 1..=20 {
            let mut expected = 1;
            for i in 1..=n {
                expected *= i;
            }
            assert_eq!(factorial(n), expected);
        }
    }

    #[test]
    pub fn test_custom_combination() {
        for n in 1..=10 {
            for r in 1..=n {
                let expected = factorial(n) / (factorial(r) * factorial(n - r));
                assert_eq!(combination(n, r), expected);
            }
        }
    }
}
