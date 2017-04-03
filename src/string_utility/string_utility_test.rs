use std::str;
use string_utility::string_utility::*;

#[test]
fn test_split() {
    let sentence: Vec<String> = split(&"this is a sentence".to_string(), &' ');
    assert_eq!(*sentence[0], "this".to_string());
    assert_eq!(*sentence[1], "is".to_string());
    assert_eq!(*sentence[2], "a".to_string());
    assert_eq!(*sentence[3], "sentence".to_string());
}

#[test]
fn test_into_string() {
    let characters: &[char] = &['t', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', ' ', 't', 'e', 's', 't'];
    assert_eq!(into_string(characters), "this is a test".to_string());
}

#[test]
fn test_substring() {
    let until_cat: String = substring(&"this is a sentence cat", &0, &18);
    assert_eq!(until_cat, "this is a sentence".to_string());
}