use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::scalar::Scalar;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticMatrix<T, const R: usize, const C: usize>
where
    T: Scalar,
{
    data: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> StaticMatrix<T, R, C>
where
    T: Scalar,
{
    pub fn new(data: [[T; C]; R]) -> Self {
        Self { data }
    }

    pub fn row(&self, i: usize) -> &[T; C] {
        &self.data[i]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [T; C] {
        &mut self.data[i]
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        assert!(i < R);
        assert!(j < C);
        &self.data[i][j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        assert!(i < R);
        assert!(j < C);
        &mut self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        assert!(i < R);
        assert!(j < C);
        self.data[i][j] = value;
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.get(i, j)
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.get_mut(i, j)
    }
}

impl<T, const R: usize, const C: usize> StaticMatrix<T, R, C>
where
    T: Scalar,
{
    pub fn identity() -> Self {
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            data[i][i] = T::default();
        }
        Self { data }
    }

    pub fn transpose(&self) -> StaticMatrix<T, C, R> {
        let mut data = [[T::default(); R]; C];
        for i in 0..R {
            for j in 0..C {
                data[j][i] = self.data[i][j];
            }
        }
        StaticMatrix::new(data)
    }

    pub fn dot<const C2: usize>(&self, matrix: StaticMatrix<T, C, C2>) -> StaticMatrix<T, R, C2> {
        let mut data = [[T::default(); C2]; R];
        for i in 0..R {
            for j in 0..C2 {
                for k in 0..C {
                    data[i][j] = data[i][j] + self.data[i][k] * matrix.data[k][j];
                }
            }
        }
        StaticMatrix::new(data)
    }

    pub fn scale(&self, scalar: T) -> Self {
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] * scalar;
            }
        }
        Self { data }
    }
}

impl<T, const R: usize, const C: usize> Default for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    fn default() -> Self {
        Self::new([[T::default(); C]; R])
    }
}

impl<T, const R: usize, const C: usize> Add for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T, const R: usize, const C: usize> Sub for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T, const R: usize, const C: usize, const C2: usize> Mul<StaticMatrix<T, C, C2>>
    for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    type Output = StaticMatrix<T, R, C2>;

    fn mul(self, other: StaticMatrix<T, C, C2>) -> Self::Output {
        self.dot(other)
    }
}

impl<T, const R: usize, const C: usize> Mul<T> for StaticMatrix<T, R, C>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        self.scale(scalar)
    }
}

// TODO: Implement DynamicMatrix
pub struct DynamicMatrix<T> {
    data: Vec<Vec<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_matrix() {
        let a = StaticMatrix::<f32, 2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = StaticMatrix::<f32, 2, 2>::new([[5.0, 6.0], [7.0, 8.0]]);
        let c = a + b;
        assert_eq!(c.data, [[6.0, 8.0], [10.0, 12.0]]);
        let d = a - b;
        assert_eq!(d.data, [[-4.0, -4.0], [-4.0, -4.0]]);
        let e = a * b;
        assert_eq!(e.data, [[19.0, 22.0], [43.0, 50.0]]);
        let f = a * 2.0;
        assert_eq!(f.data, [[2.0, 4.0], [6.0, 8.0]]);
        let g = a.transpose();
        assert_eq!(g.data, [[1.0, 3.0], [2.0, 4.0]]);
    }

    #[test]
    fn test_different_sized_matric_mul() {
        let a = StaticMatrix::<f32, 2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = StaticMatrix::<f32, 2, 3>::new([[5.0, 6.0, 7.0], [8.0, 9.0, 10.0]]);
        let c = a * b;
        assert_eq!(c.data, [[21.0, 24.0, 27.0], [47.0, 54.0, 61.0]]);
    }
}
