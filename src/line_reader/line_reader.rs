use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std;

use line_reader::file_line_collection::FileLineCollection;
use line_reader::file_line::FileLine;

pub struct FileLineReader {
    pub file_name: String
}

impl FileLineReader {
    pub fn read(&self) -> FileLineCollection {

        let mut file: File = match File::open(&self.file_name) {
            Ok(file) => file,
            _ => {
                let current_directory = std::env::current_dir().unwrap();
                panic!(format!("{} is not a valid file in {}", self.file_name, current_directory.into_os_string().into_string().unwrap()));
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents);

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