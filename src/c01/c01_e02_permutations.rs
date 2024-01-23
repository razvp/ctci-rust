use std::collections::HashMap;


fn is_permutation_1(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false; }

    let mut hm: HashMap<_, isize> = HashMap::new();
    for i in 0..s1.len() {
        let entry = hm.entry(s1.as_bytes()[i]).or_insert(0);
        *entry += 1;

        let entry = hm.entry(s2.as_bytes()[i]).or_insert(0);
        *entry -= 1;
    }

    hm.values().sum::<isize>() == 0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_1() {
       let s1 = "abcd".to_string(); 
       let s2 = "bcad".to_string(); 

        assert!(is_permutation_1(&s1, &s2));
        assert!(!is_permutation_1(&s1, &s2));
    }
}
