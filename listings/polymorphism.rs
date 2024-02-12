// Here we define a trait for converting a type to a string.
trait Show {
    fn show(&self) -> String;
}

// Then we can implement it for some types.
impl Show for Point {
    fn show(&self) -> String {
        format!("({}, {})", self.x(), self.y())
    }
}

// Traits can also be automatically derived in some cases.
#[derive(Debug)]
struct Point3D(i32, i32, i32);

// Traits can also be used as bounds for generic types.
fn print_showable<T: Show>(s: T) {
    println!("{}", s.show());
}