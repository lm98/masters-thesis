// Here we define a simple sum type
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// This function returns an Option that contains a reference to the first element of the list
fn head(list: &List) -> Option<&i32> {
    // In Rust, we can use pattern matching to destructure a sum type
    match list {
        List::Cons(value, _) => Some(value),
        List::Nil => None,
    }
}