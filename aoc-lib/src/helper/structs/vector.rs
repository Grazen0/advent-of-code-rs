use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::Float;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Float> Vector2D<T> {
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y + other.y
    }

    pub fn norm_sq(self) -> T {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn norm(self) -> T {
        self.norm_sq().sqrt()
    }

    pub fn distance_sq_to(self, other: Self) -> T {
        (self - other).norm_sq()
    }

    pub fn distance_to(self, other: Self) -> T {
        (self - other).norm()
    }

    pub fn angle_xy(self) -> T {
        T::atan2(self.y, self.x)
    }

    pub fn rotated_by(self, angle: T) -> Self {
        let sin = T::sin(angle);
        let cos = T::cos(angle);

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T>> Add<Vector2D<T>> for Vector2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vector2D<T>> for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub<Vector2D<T>> for Vector2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Vector2D<T>> for Vector2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
