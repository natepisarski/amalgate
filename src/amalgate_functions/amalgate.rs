use line_reader::line_reader::*;
use line_reader::file_line_collection::*;
use line_reader::file_line::*;

use string_utility::string_utility::*;
use std;

pub enum BeforeOrAfter {
    Before,
    After
}

pub struct AmalgateFunctions {
    pub before_or_after: BeforeOrAfter
}

struct InnerAmalgateFunctions {

}
impl InnerAmalgateFunctions {
    pub fn import_at_point(existing_lines: &Vec<String>, filename: &str) -> Vec<String> {
        let file_contents = FileLineReader {file_name: filename.to_string()};
        let file_lines = file_contents.read().lines;
        let mut resultant_lines: Vec<String> = Vec::new();
        for line in existing_lines {
            resultant_lines.push(line.to_string());
        }
        for line in file_lines {
            resultant_lines.push(line.line_text);
        }
        return resultant_lines;
    }
}
impl AmalgateFunctions {
    pub fn contains_amalgate_function(&self, line: &str) -> bool {
        line.contains("=") && !line.contains(" = ") && !line.contains("==")
    }

    pub fn amalgate_function_arguments(&self, line: &FileLine) -> Result<Vec<String>, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            let mut all_words_but_first: Vec<String> = Vec::new();
            all_words_but_first.extend_from_slice(&words[1..]);
            let mut arguments: Vec<String> = Vec::new();
            for word in all_words_but_first {
                arguments.push(word.clone());
            }
            return Ok(arguments);
        }
        return Err(format!("[{}, line {}]: Cannot find a valid amalgate function at point.",
                           line.file_name, line.line_number));
    }

    pub fn amalgate_function_name(&self, line: &FileLine) -> Result<String, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            for word in words {
                if word.starts_with("=") {
                    return Ok(substring(&word, &1, &((word.len() - 1) as u32))) // [TODO|]: This should not remain 1, rather the index of =
                }
            }
        } else {
            return Err(format!("The provided line, {}, does not contain an amalgate function.", line.line_number));
        }
        return Err(format!("Another error has occurred processing amalgate functions"))
    }

    /// Desugars an amalgate file. This will leave us with a script that
    /// only contains amalgate module calls (i.e, remote, user defined, or built-in) that must
    /// run at runtime.
    pub fn desugar(&self, filename: &str) -> Vec<String> {
        let lines: FileLineCollection = FileLineReader {file_name: String::from(filename)}.read();
        let mut result_lines: Vec<String> = Vec::new();
        for line in lines.lines {
            let file_line: FileLine = line;
            if self.contains_amalgate_function(&file_line.line_text) {
                let function_name: String = self.amalgate_function_name(&file_line).unwrap();
                match self.amalgate_function_name(&file_line).unwrap().as_ref() {
                    "import" => {
                        let arguments = self.amalgate_function_arguments(&file_line);
                        let filename = arguments.unwrap()[0].to_string();
                        result_lines = InnerAmalgateFunctions::import_at_point(&result_lines, &filename);
                    }
                    &_ => {println!("[line {}]: We couldn't find {}. Skipping for now.", file_line.line_number, function_name);}
                }
            } else {
                result_lines.push(file_line.line_text);
            }
        }
        return result_lines;
    }
}

pub fn transpile(filename: &str) -> () {
    let function_store: AmalgateFunctions = AmalgateFunctions {before_or_after: BeforeOrAfter::After};
    let desugared = function_store.desugar(filename);
    for line in desugared {
        println!("{}", line);
    }
}