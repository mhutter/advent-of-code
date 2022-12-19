use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub},
};

/// A point on a 2D coordinate system
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct GridCoord<T> {
    pub x: T,
    pub y: T,
}

impl<T> GridCoord<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for GridCoord<T> {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T: Display> Display for GridCoord<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T> Add for GridCoord<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for GridCoord<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for GridCoord<T>
where
    T: Sub<Output = T> + PartialOrd + Display,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.x < rhs.x || self.y < rhs.y {
            panic!("Subtracting {rhs} from {self} would underflow");
        }

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
