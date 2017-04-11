/// Splits a string reference based on a character. The character is not included in any of the
/// returned string pieces.
pub fn split<'a>(line: &'a str, split_char: &'a char) -> Vec<String> {
    let mut current_collection: Vec<String> = Vec::new();
    let mut current_string: String = String::new();

    for char in line.chars() {

        // We've hit the char we want to split on
        if char == *split_char {

            // To prevent split("hello      there", ' ') from getting confused
            if current_string.len() > 0 {
                current_collection.push(current_string.clone());
                current_string = String::new();
            }
        }

        // We want to push everything else to the current buffer
        if char != *split_char {
            current_string.push(char);
        }
    }

    // Our last buffer has data we need to push onto the collection
    if current_string.len() > 0 {
        current_collection.push(current_string);
    }
    return current_collection;
}

/// Turns a slice of char reference into a String.
/// P.S: One day, .collect<>() will probably serve this purpose. Today is not that day.
pub fn into_string(chars: &[char]) -> String {
    let mut collector: String = String::new();
    for character in chars {
        collector.push(character.clone());
    }
    return collector.clone();
}

/// Returns a substring of the given string, provided with
/// a starting position (starting at 0) and a length. i.e, how many characters the string should have.
pub fn substring(line: &str, start_index: &u32, length: &u32) -> String { // [TODO]: Clean up this usize vs u8 business
    let characters: Vec<char> = line.chars().skip(*start_index as usize).take(*length as usize).collect();
    return into_string(characters.as_slice());
}