use std::ops::{Add, Index, IndexMut, Mul};

use crate::{matrix::StaticMatrix, scalar::Scalar};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector<T, const N: usize>
where
    T: Scalar,
{
    data: StaticMatrix<T, 1, N>,
}

impl<T, const N: usize> Vector<T, N>
where
    T: Scalar,
{
    pub fn new(data: [T; N]) -> Self {
        Self {
            data: StaticMatrix::new([data]),
        }
    }

    pub fn get(&self, i: usize) -> &T {
        assert!(i < N);
        &self.data[(0, i)]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        assert!(i < N);
        &mut self.data[(0, i)]
    }

    pub fn set(&mut self, i: usize, value: T) {
        assert!(i < N);
        self.data[(0, i)] = value;
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        self.get(i)
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: Scalar,
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.get_mut(i)
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = StaticMatrix::new([[T::default(); N]; 1]);

        for i in 0..N {
            data[(0, i)] = self.data[(0, i)] + rhs.data[(0, i)];
        }

        Self { data }
    }
}

impl<T, const N: usize> Add<T> for Vector<T, N>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut data = StaticMatrix::new([[T::default(); N]; 1]);

        for i in 0..N {
            data[(0, i)] = self.data[(0, i)] + rhs;
        }

        Self { data }
    }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = StaticMatrix::new([[T::default(); N]; 1]);

        for i in 0..N {
            data[(0, i)] = self.data[(0, i)] * rhs.data[(0, i)];
        }

        Self { data }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = StaticMatrix::new([[T::default(); N]; 1]);

        for i in 0..N {
            data[(0, i)] = self.data[(0, i)] * rhs;
        }

        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([4, 5, 6]);

        let c = a + b;
        assert_eq!(c[0], 5);
        assert_eq!(c[1], 7);
        assert_eq!(c[2], 9);

        let d = a * b;
        assert_eq!(d[0], 4);
        assert_eq!(d[1], 10);
        assert_eq!(d[2], 18);
    }
}
