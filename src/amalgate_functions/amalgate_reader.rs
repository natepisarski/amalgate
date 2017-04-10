use std;
use amalgate_functions::amalgate::*;

use amalgate_functions::Import::Import;

use line_reader::file_line::FileLine;
use line_reader::file_line_collection::FileLineCollection;
use line_reader::line_reader::FileLineReader;
use string_utility::string_utility::*;

pub struct AmalgateReader {
    pub single_line_functions: Vec<Box<AmalgateFunction>>,
    pub multi_line_functions: Vec<Box<AmalgateMultilineFunction>>
}

impl AmalgateReader {
    pub fn initialize_single_line_functions(&mut self) {
        self.single_line_functions = vec![Import{}];
    }

    pub fn initialize_multi_line_functions(&mut self) {
        self.multi_line_functions = vec![];
    }

    pub fn get_function<T>(&self, call_result: &String) -> Result<T, String>
        where T: Fn(Vec<String>, Vec<String>) -> Vec<String> {
        for function in self.single_line_functions {
            let single_line_function: AmalgateFunction = function;
            if single_line_function.used_on(call_result) {
                Ok(return single_line_function.call);
            }
        }
        return Err("No suitable single-line function was found");
    }

    pub fn contains_amalgate_function(&self, line: &str) -> bool {
        // == is reserved for amalgate modules
        line.trim().starts_with("=") && !line.trim().starts_with("==")
    }

    /// Multiline functions
    pub fn is_multiline_function(&self, line: &str) -> bool {
        let line_content = line.trim();

        if line.len() < 1 {
            return false;
        }

        if line_content[0] == '=' && line_content[line.len() - 1] == '=' {
            return true;
        }

        return false;
    }

    pub fn get_multiline_arguments(&self, line: &str) -> (Vec<String>, Vec<String>) {
        (vec![], vec![])
    }

    pub fn amalgate_function_arguments(&self, line: &FileLine) -> Result<Vec<String>, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            let mut all_words_but_first: Vec<String> = Vec::new();
            all_words_but_first.extend_from_slice(&words[1..]);
            let mut arguments: Vec<String> = Vec::new();
            for word in all_words_but_first {
                arguments.push(word.replace("=", "").clone());
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
                    let mut name: String = (substring(&word, &1, &((word.len() - 1) as u32)));
                    return Ok(name.as_str().replace("=", ""))
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
        let lines: FileLineCollection = FileLineReader { file_name: String::from(filename) }.read();
        let mut result_lines: Vec<String> = Vec::new();
        for line in lines.lines {
            let file_line: FileLine = line;
            if self.contains_amalgate_function(&file_line.line_text) {
                let function_name: String = self.amalgate_function_name(&file_line).unwrap();
                let function_arguments: Vec<String> = self.amalgate_function_arguments(&file_line).unwrap();
                let used_function = self.get_single_line_function(&function_name).unwrap();
                result_lines = self.used_function.call(result_lines, function_arguments);
            } else {
                result_lines.push(file_line.line_text);
            }
        }
        return result_lines;
    }

    pub fn transpile(filename: &str) -> () {
        let amalgate_reader: AmalgateReader;
        let desugared = amalgate_reader.desugar(filename);
        for line in desugared {
            println!("{}", line);
        }
    }
}