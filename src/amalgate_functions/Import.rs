use amalgate_functions::amalgate::*;
use line_reader::line_reader::FileLineReader;
use line_reader::file_line_collection::FileLineCollection;
use line_reader::file_line::FileLine;

pub struct Import {

}
impl AmalgateFunction for Import {

    /// Amalgate Function
    /// Calling Convention: =import file1 file2 file3
    /// Imports the ordered files at the calling point
    fn call (&self, existing_lines: &Vec<String>, arguments: Vec<String>) -> Vec<String> {
        let mut file_lines: Vec<String> = Vec::new();
        let mut resultant_lines: Vec<String> = Vec::new();

        // Pull all of the lines from the files in order
        for filename in arguments {
            let file_contents = FileLineReader { file_name: filename.to_string() }.read().lines();
            for line in file_contents {
                file_lines.push(line.clone());
            }
        }

        // Adds all of the existing lines in the file to this buffer
        for line in existing_lines {
            resultant_lines.push(line.to_string());
        }

        // Adds the calculated file lines
        for line in file_lines {
            resultant_lines.push(line.line_text);
        }

        return resultant_lines;
    }

    fn used_on(&self, item: &String) -> bool {
        if item.eq("import") || item.eq("imp");
    }
}