use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use num_traits::PrimInt;

/** A point in a 2D space. It can also be used to index `Grid`. */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn origin() -> Self {
        Self::new(0, 0)
    }

    pub const fn unit_up() -> Self {
        Self::new(0, -1)
    }

    pub const fn unit_down() -> Self {
        Self::new(0, 1)
    }

    pub const fn unit_left() -> Self {
        Self::new(-1, 0)
    }

    pub const fn unit_right() -> Self {
        Self::new(1, 0)
    }

    pub fn up(&self) -> Self {
        self + Self::unit_up()
    }

    pub fn down(&self) -> Self {
        self + Self::unit_down()
    }

    pub fn left(&self) -> Self {
        self + Self::unit_left()
    }

    pub fn right(&self) -> Self {
        self + Self::unit_right()
    }

    pub fn manhattan_dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn euclidean_dist(&self, other: &Self) -> f32 {
        ((self.x as f32 - other.x as f32).powi(2) + (self.y as f32 - other.y as f32).powi(2)).sqrt()
    }

    pub fn neighbors(&self) -> [Self; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }

    pub fn neighbors_diag(&self) -> [Self; 8] {
        [self.up(), self.down(), self.left(), self.right(),
         self.up().left(), self.up().right(), self.down().left(), self.down().right()]
    }
}

//////////////////////////////////////////////////////////////////////////

impl<I: PrimInt + Display> From<(I, I)> for Point {
    fn from((x, y): (I, I)) -> Self {
        let x = x.to_i32().unwrap_or_else(|| panic!("x value too large: {x}"));
        let y = y.to_i32().unwrap_or_else(|| panic!("y value too large: {y}"));
        Self::new(x, y)
    }
}

impl From<Point> for (i32, i32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

//////////////////////////////////////////////////////////////////////////

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Point) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

//////////////////////////////////////////////////////////////////////////

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Point> for Point {
    type Output = Self;

    fn sub(self, rhs: &Point) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = *self - rhs;
    }
}

//////////////////////////////////////////////////////////////////////////

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y)
    }
}

impl Mul<&Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

//////////////////////////////////////////////////////////////////////////

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}
