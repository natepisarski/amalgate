#[allow(unused_imports)]
use std::borrow::*;
#[allow(unused_imports)]
use std::rc::*;

use amalgate_functions::amalgate::*;
use amalgate_functions::import::Import;

use line_reader::file_line::FileLine;
use line_reader::file_line_collection::FileLineCollection;
use line_reader::line_reader::FileLineReader;
use string_utility::string_utility::*;

pub struct AmalgateReader {
    pub single_line_functions: Vec<Rc<AmalgateFunction>>,
    pub multi_line_functions: Vec<Rc<AmalgateMultilineFunction>>
}

impl AmalgateReader {
    pub fn initialize_single_line_functions(&mut self) {
        self.single_line_functions = vec![Rc::new(Import {})];
    }

    #[allow(dead_code)]
    pub fn initialize_multi_line_functions(&mut self) {
        self.multi_line_functions = vec![];
    }

    pub fn get_function(&self, call_result: &String) -> Rc<AmalgateFunction> {
        for function in self.single_line_functions.as_slice() {
            let single_line_function: Rc<AmalgateFunction> = function.clone();
            if (*single_line_function).used_on(call_result) {
                return single_line_function;
            } else {
            }
        }
        panic!("Unsuitable amalgate function: |{}|", call_result);
    }

    pub fn get_multiline_function(&self, call_result: &String) -> Rc<AmalgateMultilineFunction> {
        for function in self.multi_line_functions.as_slice() {
            let multi_line_function: Rc<AmalgateMultilineFunction> = function.clone();
            if (*multi_line_function).used_on(call_result) {
                return multi_line_function;
            } else {
            }
        }
        panic!("Unsuitable amalgate multiline function: |{}|", call_result);
    }

    pub fn contains_amalgate_function(&self, line: &str) -> bool {
        // == is reserved for amalgate modules
        line.trim().starts_with("=") && !line.trim().starts_with("==")
    }

    pub fn contains_multiline_function(&self, line: &str) -> bool {
        let line_content = line.trim();

        if line.len() < 1 {
            return false;
        }

        if getchar(line_content, 0) == '=' && getchar(line_content, 1) == '=' {
            return true;
        }

        return false;
    }

    /// Pulls an amalgate multiline function call, starting at the first line of rest_lines
    pub fn amalgate_multiline_function_argumnets(&self, rest_lines: Vec<FileLine>) -> (Vec<String>, Vec<String>) {
        let mut all_lines: Vec<FileLine> = rest_lines;
        let mut home_line: FileLine = all_lines.as_slice()[0].clone();
        home_line.line_text = substring(&home_line.line_text, &1, &(home_line.line_text.len() as u32));
        let first_line_arguments = match self.amalgate_function_arguments(&home_line) {
            Ok(x) => x,
            Err(x) => {println!("{}", x); panic!("LORD JESUS");}
        };

        let mut body_lines: Vec<String> = vec![];


        for line in all_lines.split_off(1) {
            let line_as_string: String = line.line_text.to_string();

            // Multi-Line functions are terminated with lines just consisting of '=='
            if line_as_string.eq(&"==".to_string()) {
                break;
            }
            body_lines.push(line_as_string.clone());
        }
        return (first_line_arguments, body_lines);
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
        return Err(format!("[{}, line {}, text: |{}|]: Cannot find a valid amalgate function at point.",
                           line.file_name, line.line_number, line.line_text));
    }

    pub fn amalgate_function_name(&self, line: &FileLine) -> Result<String, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            for word in words {
                if word.starts_with("=") {
                    let name: String = substring(&word, &1, &((word.len() - 1) as u32));
                    return Ok(name.as_str().replace("=", ""))
                }
            }
        } else {
            return Err(format!("The provided line, {}, does not contain an amalgate function.", line.line_number));
        }
        return Err(format!("Another error has occurred processing amalgate functions"))
    }

    pub fn amalgate_multiline_function_name(&self, line: &FileLine) -> Result<String, String> {
        if self.contains_amalgate_function(&line.line_text) {
            let words = split(&line.line_text, &' ');
            for word in words {
                if word.starts_with("=") {
                    let name: String = substring(&word, &2, &((word.len() - 1) as u32));
                    return Ok(name.as_str().replace("=", ""))
                }
            }
        } else {
            return Err(format!("The provided line, {}, does not contain an amalgate function.", line.line_number));
        }
        return Err(format!("Another error has occurred processing amalgate functions"))
    }

    /// Completely desugars an amalgate file. Desugaring is the largest step in the amalgate process.
    /// All preprocessing, dependency checking / injection, etc. Should happen here. Works automatically
    /// based on the members of self.singleLineFunctions and self.multiLineFunctions.
    pub fn desugar(&self, filename: &str) -> Vec<String> {
        let lines: FileLineCollection = FileLineReader { file_name: String::from(filename) }.read();

        let mut multiline_collection_buffer: Vec<FileLine> = vec![];
        let mut multiline_collecting = false;
        let mut current_multiline_name: String = "".to_string();
        let mut current_multiline_arguments: (Vec<String>, Vec<String>);
        let mut current_multiline_function: Rc<AmalgateMultilineFunction>;
        let mut result_lines: Vec<String> = Vec::new();

        // Go through every line
        for line in lines.lines {

            // Used for auto-complete in Rust IDEs
            let file_line: FileLine = line;

            // This line denotes an amalgate single function
            if self.contains_amalgate_function(&file_line.line_text) {
                // Grab its name, arguments, and implementation
                let function_name: String = self.amalgate_function_name(&file_line).unwrap();
                let function_arguments: Vec<String> = self.amalgate_function_arguments(&file_line).unwrap();
                let used_function = self.get_function(&function_name);

                // Call it
                result_lines = used_function.call(&result_lines, function_arguments) as Vec<String>;
                continue;
            } else {
                if self.contains_multiline_function(&file_line.line_text) && !multiline_collecting {
                    current_multiline_name = self.amalgate_multiline_function_name(&file_line).unwrap();
                    current_multiline_function = self.get_multiline_function(&current_multiline_name);
                    multiline_collecting = true;
                    multiline_collection_buffer.push(file_line.clone());
                }

                if file_line.line_text.eq(&"==".to_string()) && multiline_collecting {
                    multiline_collection_buffer.push(FileLine {line_text: "==".to_string(), line_number: 0, file_name: "KILL_BUFFER".to_string()});
                    multiline_collecting = false;

                    current_multiline_arguments = self.amalgate_multiline_function_argumnets(multiline_collection_buffer.clone());
                    match current_multiline_arguments {
                        (line_args, body_args) => {
                            result_lines = self.get_multiline_function(&current_multiline_name).call(&result_lines, line_args, body_args);
                        }
                    }
                }
                if multiline_collecting {
                    multiline_collection_buffer.push(file_line.clone());
                }
            }

            if !multiline_collecting {
                result_lines.push(file_line.line_text);
            }

            }
        return result_lines;
    }

    /// Complete transpilation of a file. Currently, this initializes the amalgate reader and
    /// runs a desugar on your file. However, in the future there will be steps other than desugar
    /// (like parsing and syntax checking), so transpile will not remain the entry point to the program
    /// forever.
    pub fn transpile(filename: &str) -> () {
        let mut amalgate_reader: AmalgateReader = AmalgateReader { single_line_functions: vec![], multi_line_functions: vec![] };
        amalgate_reader.initialize_single_line_functions();
        let desugared = amalgate_reader.desugar(filename);
        for line in desugared {
            println!("{}", line);
        }
    }
}