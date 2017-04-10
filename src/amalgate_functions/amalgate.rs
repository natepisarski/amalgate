use line_reader::line_reader::*;
use line_reader::file_line_collection::*;
use line_reader::file_line::*;

use amalgate_functions::Import;
use string_utility::string_utility::*;
use std;

pub trait AmalgateFunction {
    fn call(&self, existing_lines: &Vec<String>, argument: Vec<String>) -> Vec<String>;

    fn used_on(&self, name: &String) -> bool;
}

pub trait AmalgateMultilineFunction {
    fn call(existing_lines: &Vec<String>, argument: Vec<String>, body: Vec<String>) -> Vec<String>
        where Self: Sized;

    fn used_on(name: &String) -> bool
        where Self: Sized;
}
/*
impl std::fmt::Display for AmalgateFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Amalgate Function");
    }
}

impl std::error::Error for AmalgateFunction {
    fn description(&self) -> &str { "Error in Amalgate Function" }
}

impl std::fmt::Display for AmalgateMultilineFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Amalgate Multiline Function");
    }
}

impl std::error::Error for AmalgateMultilineFunction {
    fn description(&self) -> &str { "Error in Amalgate Multiline Function" }
}
*/
pub enum BeforeOrAfter {
    Before,
    After
}