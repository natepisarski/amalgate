use line_reader::line_reader::*;
use line_reader::file_line_collection::*;
use line_reader::file_line::*;

use string_utility::string_utility::*;
use std;

pub enum BeforeOrAfter {
    Before,
    After
}

pub struct amalgate_functions {
    before_or_after: BeforeOrAfter
}

impl amalgate_functions {
    pub fn contains_amalgate_function(&self, line: &str) -> bool {
        line.contains("=") && !line.contains(" = ")
    }

    fn amalgate_function_name(&self, line: &FileLine) -> Result<String, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            for word in words {
                if word.starts_with("=") {
                    return Ok(substring(&word, &0, &((word.len() - 1) as u32))) // [TODO|]: This should not remain 0, rather the index of =
                }
            }
        } else {
            return Err(format!("The provided line, {}, does not contain an amalgate function.", line.line_number));
        }
        return Err(format!("Another error has occurred processing amalgate functions"))
    }

    pub fn import(&self, filename: &str) -> () {
        let lines: FileLineCollection = FileLineReader {file_name: String::from(filename)}.read();
    }
}