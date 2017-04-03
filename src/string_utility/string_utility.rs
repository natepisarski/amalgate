use std::str::*;
use std;

pub fn split<'a>(line: &'a str, split_char: &'a char) -> Vec<String> {
    let mut current_collection: Vec<String> = Vec::new();
    let mut current_string: String = String::new();

    for char in line.chars() {
        if char == *split_char {
            if current_string.len() > 0 {
                current_collection.push(current_string.clone());
                current_string = String::new();
            }
        }
        if char != *split_char {
            current_string.push(char);
        }
    }
    if current_string.len() > 0 {
        current_collection.push(current_string);
    }
    return current_collection;
}

pub fn into_string(chars: &[char]) -> String {
    let mut collector: String = String::new();
    for character in chars {
        collector.push(character.clone());
    }
    return collector.clone();
}

pub fn substring(line: &str, start_index: &u32, length: &u32) -> String { // [TODO]: Clean up this usize vs u8 business
    let characters: Vec<char> = line.chars().skip(*start_index as usize).take(*length as usize).collect();
    return into_string(characters.as_slice());
}