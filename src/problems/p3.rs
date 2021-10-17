use std::iter;

fn next_prime(n: i64) -> i64 {
    if n % 2 == 0 {
        2
    } else {
        let m = (n as f64 - 0.5).sqrt() as i64;
        (3..=m).step_by(2).find(|x| n % x == 0).unwrap_or(n)
    }
}

fn largest_prime_factor(mut num: i64) -> i64 {
    iter::from_fn(|| {
        (num > 1).then(|| {
            let prime = next_prime(num);
            num /= prime;
            prime
        })
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factorization() {
        assert_eq!(5, next_prime(13_195));
        assert_eq!(7, next_prime(2_639));
        assert_eq!(13, next_prime(377));
        assert_eq!(29, next_prime(29));
    }

    #[test]
    fn solution() {
        assert_eq!(6_857, largest_prime_factor(600_851_475_143));
    }
}
