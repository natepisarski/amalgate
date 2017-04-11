use line_reader::file_line::FileLine;

/// Represents a file. The only difference than using a file utility is
/// this associates metadata with each line.
pub struct FileLineCollection {
    pub lines: Vec<FileLine>
}

impl FileLineCollection { }