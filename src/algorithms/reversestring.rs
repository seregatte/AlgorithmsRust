// Solution 1
// pub fn reverse(s: &str) -> String {
//     s.chars().rev().collect::<String>()
// }
// Solution 2
// pub fn reverse(s: &str) -> String {
//     s.chars().fold("".to_string(), |mut reversed, c| {
//         reversed.insert(0 , c);
//         reversed
//     })
// }



pub fn reverse(s: &str) -> String {
    s.chars().fold("".to_string(), |reversed, c| {
        format!("{}{}", c, reversed)
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