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