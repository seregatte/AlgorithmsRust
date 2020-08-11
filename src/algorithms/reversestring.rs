// pub fn reverse(s: &str) -> String {
//     s.chars().rev().collect::<String>()
// }

pub fn reverse(s: &str) -> String {
    s.chars().fold("".to_string(), |mut reversed, c| {
        reversed.insert(0 , c);
        reversed
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("Opa"), "apO");
    }
}