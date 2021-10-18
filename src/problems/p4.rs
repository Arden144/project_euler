fn is_palindrome(val: &i32) -> bool {
    let str = val.to_string();
    let (h1, h2) = str.split_at(str.len() / 2);
    h1 == h2.chars().rev().collect::<String>()
}

fn largest_palindrone_product() -> i32 {
    (100..=990)
        .rev()
        .step_by(11)
        .filter_map(|x| (100..=999).rev().map(|y| x * y).find(is_palindrome))
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
