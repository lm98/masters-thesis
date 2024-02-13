// A simple closure that adds 5 to a number. Note that the type
// parameter of x can be omitted.
let add_five = |x: i32| x + 5;

// An example of a closure that captures its environment
fn add_prefix() -> impl Fn(&str) -> String {
    let prefix = "Mr.";
    // This closure needs to take ownership of prefix, so it isn't freed when the scope of the add_prefix function ends.
    move |string: &str| format!{"{} {}", prefix, string}
}