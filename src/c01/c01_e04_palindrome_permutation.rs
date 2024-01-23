use std::collections::HashMap;

fn is_palindrom_permutation(s: &str) -> bool {
    let mut hm: HashMap<char, usize> = HashMap::new();
    s.to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .for_each(|c| *hm.entry(c).or_insert(0) += 1);

    dbg!(&hm);
    let mut found_odd = false;
    for count in hm.values() {
        if count % 2 != 0 {
            if found_odd {
                return false;
            } else {
                found_odd = true;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_perm() {
        assert!(is_palindrom_permutation("Tact Coa"));
    }
}
