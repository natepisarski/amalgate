/// The basic function in the amalgate system. It provides an interface to create a single new
/// piece of functionality for amalgate. Each function 'lives' on its own and will be dispatched
/// when the used_on predicate holds true for a name.
pub trait AmalgateFunction {

    /// 'call' invokes the function. Given the preprocessed lines that have either been read or
    /// calculated before the calling site, and a list of arguments that control what it will do.
    /// Returns control at the index of the new calling site.
    fn call(&self, existing_lines: &Vec<String>, argument: Vec<String>) -> Vec<String>;

    /// 'used_on' is a simple Predicate that returns true when a name should trigger this function.
    /// For instance, =import listens for an amalgate function starting with 'imp', since no others
    /// do right now.
    fn used_on(&self, name: &String) -> bool;
}

/// An Amalgate Multiline function works similarly to an Amalgate Function, but it also takes a body.
/// Control flow of a multiline function is distinct in turning lines WITHOUT AMALGATE CALLS into
/// amalgate arguments.
pub trait AmalgateMultilineFunction {

    /// Given the lines that have come before the calling site, the list of arguments, and the lines
    /// constituting the body, return a value. Control flow is returned to the end of the multi-line's
    /// body declaration.
    fn call(existing_lines: &Vec<String>, argument: Vec<String>, body: Vec<String>) -> Vec<String>
        where Self: Sized;

    /// Works exactly the same as used_on does for AmalgateFunction, except the reader will
    /// only interpret calls in the form:
    ///
    /// =myFunction arg1 arg2=
    /// bodyLine1
    /// bodyLine2
    /// ==
    ///
    /// to be multiline functions.
    fn used_on(name: &String) -> bool
        where Self: Sized;
}