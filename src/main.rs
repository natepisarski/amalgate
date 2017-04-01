mod line_reader;
use line_reader::line_reader::FileLineReader;


fn main() {
    let result = FileLineReader {file_name: "test.sh".to_string()};
    let result_lines = result.read();
    println!("Hello, world!");
}
