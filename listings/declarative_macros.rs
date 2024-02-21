#[macro_export] // export the macro to make it visible to other modules

// macro_rules! is a macro that defines a new macro with the name and body that follows the invocation
macro_rules! vec {
    // The macro's body is similar to a match expression and can contain several (...) => { ... } cases
    // If the code passed to the macro matches the pattern, the code in the block is expanded
    // In this case, this pattern matches a list of expressions separated by commas.
    // The $() syntax is used to define a variable. In this case, $x is of type expr, which means it can match any Rust expression.
    // There are several types of variables that can be used in a macro, such as expr, ident, block, etc.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // This special syntax allows to perform a repetition over the elements of the input
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}