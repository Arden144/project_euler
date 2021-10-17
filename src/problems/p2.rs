struct Fib {
    a: i32,
    b: i32,
    limit: i32,
}

impl Fib {
    fn new(limit: i32) -> Self {
        Fib { a: 0, b: 1, limit }
    }
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let next = self.a + self.b;
        if next > self.limit {
            return None;
        };
        self.a = self.b;
        self.b = next;
        Some(next)
    }
}

fn even_fibonacci() -> i32 {
    Fib::new(4_000_000).filter(|x| x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_generator() {
        assert_eq!(
            vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89],
            Fib::new(89).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn solution() {
        assert_eq!(4_613_732, even_fibonacci());
    }
}
