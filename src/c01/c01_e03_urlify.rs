trait URLExt {
    fn urlify(self) -> Self;
}

impl URLExt for String {
    fn urlify(mut self) -> String {
        // let
        self = self.trim().to_owned();
        while let Some(index) = self.find(' ') {
            self.replace_range(index..index + 1, "%20");
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(
            "Mr John Smith ".to_string().urlify(),
            "Mr%20John%20Smith".to_string()
        );
    }
}
