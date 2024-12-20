pub mod template;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

impl std::ops::Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

pub type Grid<'a> = Vec<&'a str>;

impl std::ops::Index<&Point> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[index.0 as usize].as_bytes()[index.1 as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use super::*;

    #[test]
    fn hash_p_order() {
        let p_01 = Point(0,1);
        let p_10 = Point(1,0);

        let mut d_01 = DefaultHasher::new();
        p_01.hash(&mut d_01);

        let mut d_10 = DefaultHasher::new();
        p_10.hash(&mut d_10);

        assert_ne!(d_01.finish(), d_10.finish())
    }
}
