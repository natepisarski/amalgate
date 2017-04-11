mod line_reader;
use line_reader::line_reader::FileLineReader;

mod string_utility;

mod amalgate_functions;
use amalgate_functions::amalgate_reader::*;

/// Temporary testing main while amalgate is under heavy development
fn main() {
    let result = FileLineReader {file_name: "test.sh".to_string()};
    let result_lines = result.read();
    let mut reader: AmalgateReader = AmalgateReader{single_line_functions: vec![], multi_line_functions: vec![]};
    reader.initialize_single_line_functions();
    AmalgateReader::transpile(&"test.sh".to_string());
}
