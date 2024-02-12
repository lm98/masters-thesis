struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}