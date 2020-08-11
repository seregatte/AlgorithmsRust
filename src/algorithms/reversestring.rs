// pub fn reverse(s: &str) -> String {
//     s.chars().rev().collect::<String>()
// }

pub fn reverse(s: &str) -> String {
    let mut reversed = String::from("");
    s.chars().for_each(|c| reversed.insert(0 , c) );
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("Opa"), "apO");
    }
}