fn smallest_multiple() -> i32 {
    (2520..)
        .step_by(2520)
        .find(|x| (11..20).all(|y| x % y == 0))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(232_792_560, smallest_multiple());
    }
}
