pub fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(reverse("Opa"), "apO");
    }
}