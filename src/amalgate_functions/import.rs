use amalgate_functions::amalgate::*;
use line_reader::line_reader::FileLineReader;
use line_reader::file_line::FileLine;


/// Import is the 'base' function in Amalgate. Historically, it's the reason Amalgate was made.
/// It allows you to import bash files together, letting you split your bash code in more readable,
/// logical chunks. The advantage is that you are left with one file, whereas using bash's read
/// would leave you with a script with dependencies.
pub struct Import { }

impl AmalgateFunction for Import {

    /// Amalgate Function
    /// Calling Convention: =import file1 file2 file3
    /// Imports the ordered files at the calling point
    fn call (&self, existing_lines: &Vec<String>, arguments: Vec<String>) -> Vec<String> {
        let mut file_lines: Vec<String> = Vec::new();
        let mut resultant_lines: Vec<String> = Vec::new();

        // Pull all of the lines from the files in order
        for filename in arguments {
            let file_contents: Vec<FileLine> = FileLineReader { file_name: filename.to_string() }.read().lines;
            for line in file_contents {
                let inner_line: FileLine = line;
                let inner_line_text: String = inner_line.line_text;
                file_lines.push(inner_line_text);
            }
        }

        // Adds all of the existing lines in the file to this buffer
        for line in existing_lines {
            resultant_lines.push(line.to_string());
        }

        // Adds the calculated file lines
        for line in file_lines {
            resultant_lines.push(line);
        }

        return resultant_lines;
    }

    /// Amalgate Function
    /// Trigger on: import or imp
    fn used_on(&self, item: &String) -> bool {
        return item.eq("import") || item.eq("imp");
    }
}