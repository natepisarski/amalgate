use std::clone::Clone;
use std::fmt;
/// Represents a line from a file.
#[derive(Clone)]
pub struct FileLine {
    /// The name of the file the line originated in.
    pub file_name: String,

    /// The line number of the item, starting with 0.
    pub line_number: u32,

    /// The content of the line
    pub line_text: String
}

impl FileLine { }

impl fmt::Display for FileLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {}, {})", self.file_name, self.line_number, self.line_text);
    }
}