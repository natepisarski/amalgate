/// Represents a line from a file.
pub struct FileLine {

    /// The name of the file the line originated in.
    pub file_name: String,

    /// The line number of the item, starting with 0.
    pub line_number: u32,

    /// The content of the line
    pub line_text: String
}

impl FileLine { }