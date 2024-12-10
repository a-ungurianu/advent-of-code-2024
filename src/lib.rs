pub mod template;

#[derive(Debug)]
pub struct Point(pub usize, pub usize);

impl std::ops::Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(
            (self.0 as i32 + rhs.0) as usize,
            (self.1 as i32 + rhs.1) as usize,
        )
    }
}

pub type Grid<'a> = Vec<&'a str>;

impl std::ops::Index<&Point> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[index.0].as_bytes()[index.1]
    }
}
