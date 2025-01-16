use std::iter::Enumerate;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use num_traits::PrimInt;

use super::Point;

/** A 2D-like structure backed by a Vec. */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct GridEnumerate<'a, T> {
    iter: Enumerate<Iter<'a, T>>,
    mat: &'a Grid<T>
}

impl<T> Grid<T> {
    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Self { width, height, data }
    }

    pub fn map_from_str(string: &str, mapper: fn(char) -> T) -> Self {
        let width = string.lines().next().unwrap().len();
        let data: Vec<_> = string.chars().filter(|ch| !ch.is_whitespace()).map(mapper).collect();
        Self{ width, height: data.len() / width, data }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Point) -> Option<&T> {
        if self.is_in_bounds(pos) {
            Some(&self[pos])
        } else {
            None
        }
    }

    pub fn is_in_bounds(&self, pos: Point) -> bool {
        let bound_x = self.width() as i32;
        let bound_y = self.height() as i32;

        pos.x >= 0 && pos.y >= 0 && pos.x < bound_x && pos.y < bound_y
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn enumerate(&self) -> GridEnumerate<T> {
        GridEnumerate { mat: self, iter: self.data.iter().enumerate() }
    }

    //////////////////////////////////////////////////////////////////////////

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width(), "x index out of bounds: {x} but width is {}", self.width());
        assert!(y < self.height(), "y index out of bounds: {y} but height is {}", self.height());
        y * self.width + x
    }

    fn coords(&self, index: usize) -> Point {
        Point::new((index % self.width) as i32, (index / self.width) as i32)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn get_or(&self, pos: Point, default: T) -> T {
        self.get(pos).copied().unwrap_or(default)
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<Point> {
        /* Find the first position of the provided element, if it exists. */
        self.enumerate().find(|&x| x.1 == elem).map(|x| x.0)
    }

    pub fn find_all<'a>(&'a self, elem: &'a T) -> impl Iterator<Item = Point> + 'a {
        /* Finds the positions of all elements that are equal to the provided one */
        self.enumerate().filter(move |&x| x.1 == elem).map(|x| x.0)
    }
}

impl Grid<char> {
    pub fn from_str(string: &str) -> Self {
        Self::map_from_str(string, |c| c)
    }
}

//////////////////////////////////////////////////////////////////////////

impl<'a, T> Iterator for GridEnumerate<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, x)| (self.mat.coords(i), x))
    }
}

//////////////////////////////////////////////////////////////////////////

impl<T, I: PrimInt + Display> Index<(I, I)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")),
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &self.data[i]
    }
}

impl<T, I: PrimInt + Display> IndexMut<(I, I)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (I, I)) -> &mut Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")),
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &mut self.data[i]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        &self[(p.x, p.y)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self[(p.x, p.y)]
    }
}

//////////////////////////////////////////////////////////////////////////

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
