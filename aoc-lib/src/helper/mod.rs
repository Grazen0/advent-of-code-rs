use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod math;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index2D<T> {
    pub i: T,
    pub j: T,
}

impl<T> Index2D<T> {
    pub fn new(i: T, j: T) -> Self {
        Self { i, j }
    }
}

impl<T: Neg<Output = T>> Neg for Index2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            i: -self.i,
            j: -self.j,
        }
    }
}

impl<T: Add<Output = T>> Add for Index2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl<T: AddAssign> AddAssign for Index2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

impl<T: Sub<Output = T>> Sub for Index2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl<T: SubAssign> SubAssign for Index2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.i -= rhs.i;
        self.j -= rhs.j;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Index2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            i: self.i * rhs,
            j: self.j * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Index2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.i *= rhs;
        self.j *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Index2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            i: self.i / rhs,
            j: self.j / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Index2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.i /= rhs;
        self.j /= rhs;
    }
}
