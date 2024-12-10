pub mod template;

#[derive(Debug)]
pub struct Point(pub i32, pub i32);

impl std::ops::Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub type Grid<'a> = Vec<&'a str>;

impl std::ops::Index<&Point> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[index.0 as usize].as_bytes()[index.1 as usize]
    }
}
