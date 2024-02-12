fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello")

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}