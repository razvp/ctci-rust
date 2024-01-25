#![allow(dead_code)]

use std::{collections::HashMap, usize};

fn is_unique_solution_1(s: &str) -> bool {
    let mut h: HashMap<_, usize> = HashMap::new();
    for c in s.chars() {
        dbg!(&c);
        let entry = h.entry(c).or_default();
        if *entry != 0 {
            return false;
        }
        *entry += 1
    }

    true
}

fn is_unique_solution_2(s: &str) -> bool {
    if s.len() > 128 {
        // no more than 128 chars in alphabet
        return false;
    }

    let mut occurance_array = [false; 128];

    for c in s.chars() {
        assert!((c as usize) < 128, "not in charset");
        let c = c as usize;
        dbg!(c as usize);
        if occurance_array[c] {
            return false;
        } else {
            occurance_array[c] = true;
        }
    }

    true
}

#[test]
fn test_is_unique() {
    let s = "abcd".to_string();
    assert_eq!(is_unique_solution_1(&s), true);

    let s = "abcda".to_string();
    assert_eq!(is_unique_solution_1(&s), false);
}

#[test]
fn test_is_unique_2() {
    let s = "aA=".to_string();
    assert_eq!(is_unique_solution_2(&s), true);
    let s = "abcda".to_string();
    assert_eq!(is_unique_solution_2(&s), false);
}
