use std::fmt::{Display, Formatter};

struct Pos(i32, i32);
struct Size(i32, i32);

fn main() {
    println!("{}", Pos(0, 0));
}

impl Pos {
    fn x(&self) -> i32 { self.0 }
    fn y(&self) -> i32 { self.1 }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {})", self.x(), self.y())
    }
}

impl Size {
    fn w(&self) -> i32 { self.0 }
    fn h(&self) -> i32 { self.1 }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size({}, {})", self.w(), self.h())
    }
}
