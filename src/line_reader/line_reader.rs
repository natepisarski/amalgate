use std::io::Read;
use std::fs::File;
use std;

use line_reader::file_line_collection::FileLineCollection;
use line_reader::file_line::FileLine;

/// Class for reading a file line-by-line, and garnering metadata about each line.
pub struct FileLineReader {

    /// The name of the file that will be read.
    pub file_name: String
}

impl FileLineReader {

    /// Reads a file, and returns a collection of lines found within it.
    pub fn read(&self) -> FileLineCollection {

        let mut file: File = match File::open(&self.file_name) {
            Ok(file) => file,
            _ => {
                let current_directory = std::env::current_dir().unwrap();
                panic!(format!("{} is not a valid file in {}", self.file_name, current_directory.into_os_string().into_string().unwrap()));
            }
        };
        let mut contents = String::new();

        if let Err(_) = file.read_to_string(&mut contents) {
            panic!("{} could not be read! Make sure it exists and your user has read permissions to it.", self.file_name);
        }

        let file_lines = contents.lines();
        let mut line_vector: Vec<FileLine> = Vec::new();
        let mut counter = 0;

        for line in file_lines {
            line_vector.push(
                FileLine {file_name: self.file_name.clone(), line_number: counter, line_text: line.to_string()}
            );
            counter = counter + 1;
        }
        return FileLineCollection {lines: line_vector}
    }
}