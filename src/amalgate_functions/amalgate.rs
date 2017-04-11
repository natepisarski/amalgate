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