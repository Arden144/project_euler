fn multiple_3_5(limit: i32) -> i32 {
    (0..limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(233_168, multiple_3_5(1_000));
    }
}
