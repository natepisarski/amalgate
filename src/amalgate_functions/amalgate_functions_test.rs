#[allow(unused_imports)]
use amalgate_functions::amalgate_reader::*;

#[allow(unused_imports)]
use line_reader::file_line::FileLine;

#[test]
pub fn test_contains_function_name() {
    let function_runner: AmalgateReader = AmalgateReader { single_line_functions: vec![], multi_line_functions: vec![] };
    let should = &"=this should be a function name";

    assert!(function_runner.contains_amalgate_function(should));
}

#[test]
pub fn test_function_name() {
    let function_runner: AmalgateReader = AmalgateReader { single_line_functions: vec![], multi_line_functions: vec![] };
    let import = FileLine
            {line_text: "=import this should be import".to_string(), file_name: "any".to_string(), line_number: 5};
    assert_eq!(function_runner.amalgate_function_name(&import).unwrap(), "import".to_string());
}

#[test]
pub fn test_function_arguments() {
    let function_runner: AmalgateReader = AmalgateReader { single_line_functions: vec![], multi_line_functions: vec![] };
    let import = FileLine
        {line_text: "=import this should be import".to_string(), file_name: "any".to_string(), line_number: 5};
    let function_arguments: Vec<String> = function_runner.amalgate_function_arguments(&import).unwrap();
    assert_eq!(*function_arguments[0], "this".to_string());
    assert_eq!(*function_arguments[1], "should".to_string());
    assert_eq!(*function_arguments[2], "be".to_string());
    assert_eq!(*function_arguments[3], "import".to_string());
}

#[test]
pub fn test_multiline_function_arguments() {
    let function_runner: AmalgateReader = AmalgateReader {single_line_functions: vec![], multi_line_functions: vec![]};
    let mut mock_lines: Vec<FileLine> = vec![];
    mock_lines.push(FileLine {line_text: "==export this should be export".to_string(), file_name: "any".to_string(),
        line_number: 0});
    mock_lines.push(FileLine {line_text: "line 1 of export".to_string(), file_name: "any".to_string(),
        line_number: 0});
    mock_lines.push(FileLine {line_text: "line 2 of export".to_string(), file_name: "any".to_string(),
        line_number: 0});
    mock_lines.push(FileLine {line_text: "== tricky line".to_string(), file_name: "any".to_string(),
        line_number: 0});
    mock_lines.push(FileLine {line_text: "==".to_string(), file_name: "any".to_string(),
        line_number: 0});
    let result: (Vec<String>, Vec<String>) = function_runner.amalgate_multiline_function_argumnets(mock_lines);

    match result {
        (x, y) => {
            assert_eq!(*x[0], "this".to_string());
            assert_eq!(*x[2], "be".to_string());
            assert_eq!(*x[3], "export".to_string());
            assert_eq!(*y[0], "line 1 of export".to_string());
            assert_eq!(*y[2], "== tricky line".to_string());
        }
    }
}