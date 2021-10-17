fn is_palindrome(val: &str) -> bool {
    let (h1, h2) = val.split_at(val.len() / 2);
    h1 == h2.chars().rev().collect::<String>()
}

fn largest_palindrone_product() -> i32 {
    (100..=990)
        .rev()
        .step_by(11)
        .flat_map(|x| {
            (100..=999)
                .rev()
                .map(move |y| x * y)
                .filter(|n| is_palindrome(&n.to_string()))
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(906_609, largest_palindrone_product());
    }
}
