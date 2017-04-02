pub fn split<'a>(line: &'a str, split_char: &'a u8) -> Vec<String> {
    let mut current_collection: Vec<String> = Vec::new();
    let mut current_string: String = String::new();

    for char in line.chars() {
        if char as u8 == *split_char {
            if current_string.len() > 0 {
                current_collection.push(current_string.clone());
                current_string = String::new();
            }
        }
        current_string.push(char);
    }
    return current_collection;
}