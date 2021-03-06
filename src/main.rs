use std::fmt::{Display, Formatter};

struct Pos(i32, i32);

struct Size(i32, i32);

struct Area(Pos, Size);

fn area<T: ToArea>(t: T) -> Area { t.to_area() }

fn main() {
    println!("{}", Pos(0, 0));
    println!(
        "{}",
        area(Pos(50, 50))
    );
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

impl Area {
    fn pos(&self) -> Pos { Pos(self.0.0, self.0.1) }
    fn size(&self) -> Size { Size(self.1.0, self.1.1) }

    fn x(&self) -> i32 { self.pos().x() }
    fn y(&self) -> i32 { self.pos().x() }
    fn w(&self) -> i32 { self.size().w() }
    fn h(&self) -> i32 { self.size().h() }
}

trait ToArea {
    fn to_area(&self) -> Area;
}

impl ToArea for Pos {
    fn to_area(&self) -> Area {
        Area(
            Pos(self.x(), self.y()),
            Size(0, 0)
        )
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area({}, {}, {}, {})", self.x(), self.y(), self.w(), self.h())
    }
}
