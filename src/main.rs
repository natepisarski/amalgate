mod line_reader;
use line_reader::line_reader::FileLineReader;

mod string_utility;
use string_utility::string_utility::split;

mod amalgate_functions;
use amalgate_functions::amalgate::transpile;

fn main() {
    let result = FileLineReader {file_name: "test.sh".to_string()};
    let result_lines = result.read();
    transpile(&"test.sh".to_string());
}
