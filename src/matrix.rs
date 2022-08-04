use crate::{tuple::Tuple, util::FuzzyEq};
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix<const D: usize>
where
    [[f64; D]; D]: Sized,
{
    data: [[f64; D]; D],
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<const D: usize> From<[[f64; D]; D]> for Matrix<D> {
    fn from(data: [[f64; D]; D]) -> Self {
        Self { data }
    }
}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Self {
        Self::from([[0.0; D]; D])
    }

    pub fn identity() -> Self {
        let mut matrix = Self::new();

        for row in 0..D {
            matrix[row][row] = 1.0;
        }

        matrix
    }

    pub fn tranpose(&self) -> Self {
        let mut res: Matrix<D> = Matrix::default();

        for row in 0..D {
            for column in 0..D {
                res[column][row] = self[row][column]
            }
        }

        res
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<{ D - 1 }> {
        let mut matrix: Matrix<{ D - 1 }> = Matrix::new();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < D - 1 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < D - 1 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }

        matrix
    }
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [f64; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> FuzzyEq<Self> for Matrix<D> {
    fn fuzzy_eq(&self, other: Self) -> bool {
        for row in 0..D {
            for column in 0..D {
                if self[row][column].fuzzy_ne(other[row][column]) {
                    return false;
                }
            }
        }

        true
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl<const D: usize> Mul<Self> for Matrix<D> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut res = Matrix::new();

        for row in 0..D {
            for column in 0..D {
                res[row][column] = self[row][0] * other[0][column]
                    + self[row][1] * other[1][column]
                    + self[row][2] * other[2][column]
                    + self[row][3] * other[3][column];
            }
        }

        res
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix<3> {
    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant: f64 = 0.0;
        for col in 0..3 {
            determinant += self.cofactor(0, col) * self[0][col]
        }

        determinant
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant().fuzzy_ne(0.0)
    }

    pub fn inverse(&self) -> Matrix<3> {
        if !self.is_invertible() {
            panic!("Matrix is not invertible, but inverse was called!");
        }

        let mut matrix = Matrix::new();
        let determinant = self.determinant();

        for row in 0..3 {
            for column in 0..3 {
                let cofactor = self.cofactor(row, column);
                // transposed storage
                matrix[column][row] = cofactor / determinant;
            }
        }

        matrix
    }
}

impl Matrix<4> {
    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant: f64 = 0.0;
        for col in 0..4 {
            determinant += self.cofactor(0, col) * self[0][col]
        }

        determinant
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant().fuzzy_ne(0.0)
    }

    pub fn inverse(&self) -> Matrix<4> {
        if !self.is_invertible() {
            panic!("Matrix is not invertible, but inverse was called!");
        }

        let mut matrix = Matrix::new();
        let determinant = self.determinant();

        for row in 0..4 {
            for column in 0..4 {
                let cofactor = self.cofactor(row, column);
                // transposed storage
                matrix[column][row] = cofactor / determinant;
            }
        }

        matrix
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4> {
        let mut t = Self::identity();
        t[0][3] = x;
        t[1][3] = y;
        t[2][3] = z;

        t
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Matrix<4> {
        let t = Self::translation(x, y, z);

        t * self
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
        let mut t = Self::identity();
        t[0][0] = x;
        t[1][1] = y;
        t[2][2] = z;

        t
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Matrix<4> {
        let t = Self::scaling(x, y, z);

        t * self
    }

    pub fn rotation_x(r: f64) -> Matrix<4> {
        let mut t = Self::identity();
        t[1][1] = r.cos();
        t[1][2] = -(r.sin());
        t[2][1] = r.sin();
        t[2][2] = r.cos();

        t
    }

    pub fn rotate_x(self, r: f64) -> Matrix<4> {
        let t = Self::rotation_x(r);

        t * self
    }

    pub fn rotation_y(r: f64) -> Matrix<4> {
        let mut t = Self::identity();
        t[0][0] = r.cos();
        t[0][2] = r.sin();
        t[2][0] = -(r.sin());
        t[2][2] = r.cos();

        t
    }

    pub fn rotate_y(self, r: f64) -> Matrix<4> {
        let t = Self::rotation_y(r);

        t * self
    }

    pub fn rotation_z(r: f64) -> Matrix<4> {
        let mut t = Self::identity();

        t[0][0] = r.cos();
        t[0][1] = -(r.sin());
        t[1][0] = r.sin();
        t[1][1] = r.cos();

        t
    }

    pub fn rotate_z(self, r: f64) -> Matrix<4> {
        let t = Self::rotation_z(r);

        t * self
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
        let mut t = Self::identity();
        t[0][1] = x_y;
        t[0][2] = x_z;
        t[1][0] = y_x;
        t[1][2] = y_z;
        t[2][0] = z_x;
        t[2][1] = z_y;

        t
    }

    pub fn sheare(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
        let t = Self::shearing(x_y, x_z, y_x, y_z, z_x, z_y);

        t * self
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<4> {
        let forward = (to - from).normalize();
        let up_normalized = up.normalize();
        let left = forward.cross(up_normalized);
        let true_up = left.cross(forward);

        let orientation = Matrix::from([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;

        for row in 0..4 {
            let temp_res = self[row][0] * rhs.x
                + self[row][1] * rhs.y
                + self[row][2] * rhs.z
                + self[row][3] * rhs.w;

            match row {
                0 => x = temp_res,
                1 => y = temp_res,
                2 => z = temp_res,
                3 => w = temp_res,
                _ => panic!("Reached row size of over 4 in 4x4 matrix!"),
            }
        }

        Tuple::new(x, y, z, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_fuzzy_eq, assert_fuzzy_ne, tuple::Tuple};
    use std::f64::consts::PI;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[1][3], 8.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[2][3], 12.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][1], 14.5);
        assert_eq!(m[3][2], 15.5);
        assert_eq!(m[3][3], 16.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[1][2], -7.0);
        assert_eq!(m[2][0], 0.0);
        assert_eq!(m[2][1], 1.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_equaliy_identical_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let b = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_fuzzy_eq!(a, b);
    }

    #[test]
    fn matrix_equaliy_different_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let b = Matrix::from([
            [1.1, 2.1, 3.1, 4.1],
            [5.1, 6.1, 7.1, 8.1],
            [9.1, 10.1, 11.1, 12.1],
            [13.1, 14.1, 15.1, 16.1],
        ]);

        assert_fuzzy_ne!(a, b);
    }

    #[test]
    fn multiply_two_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        let actual = a * b;
        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);
        let actual = a * b;

        assert_fuzzy_eq!(expected, actual)
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let i: Matrix<4> = Matrix::identity();

        let expected = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let actual = a * i;
        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let i: Matrix<4> = Matrix::identity();
        let t = Tuple::new(1.0, 2.0, 3.0, 4.0);

        let expected = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let actual = i * t;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn transpose_a_matrix() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        let actual = a.tranpose();

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn transpose_identity_matrix() {
        let i: Matrix<4> = Matrix::identity();

        let expected = Matrix::identity();
        let actual = i.tranpose();

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn calculate_determinant_2x2_matrix() {
        let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

        let expected = 17.0;
        let actual = m.determinant();

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        let expected = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);
        let actual = a.submatrix(0, 2);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn submatrix_of_4x4_is3x3() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix::from([[0.0, 3.0, 0.0], [9.0, 0.0, 8.0], [0.0, 5.0, 8.0]]);
        let actual = a.submatrix(2, 1);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn calculate_the_minor_of_a_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let sub = m.submatrix(1, 0);
        let determinant = sub.determinant();
        let minor = m.minor(1, 0);

        assert_fuzzy_eq!(25.0, determinant);
        assert_fuzzy_eq!(25.0, minor);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, minor1);
        assert_fuzzy_eq!(-12.0, cofactor1);
        assert_fuzzy_eq!(25.0, minor2);
        assert_fuzzy_eq!(-25.0, cofactor2);
    }

    #[test]
    fn calculate_the_determinant_of_a_3x3_matrix() {
        let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);

        let determinant = m.determinant();

        assert_fuzzy_eq!(56.0, cofactor00);
        assert_fuzzy_eq!(12.0, cofactor01);
        assert_fuzzy_eq!(-46.0, cofactor02);

        assert_fuzzy_eq!(-196.0, determinant);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);
        let cofactor03 = m.cofactor(0, 3);

        let determinant = m.determinant();

        assert_fuzzy_eq!(690.0, cofactor00);
        assert_fuzzy_eq!(447.0, cofactor01);
        assert_fuzzy_eq!(210.0, cofactor02);
        assert_fuzzy_eq!(51.0, cofactor03);

        assert_fuzzy_eq!(-4071.0, determinant);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let determinant = m.determinant();

        assert_fuzzy_eq!(-2120.0, determinant);
        assert!(m.is_invertible());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let m = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let determinant = m.determinant();

        assert_fuzzy_eq!(0.0, determinant);
        assert!(!m.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_4x4_matrix() {
        let m = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let determinant = m.determinant();
        let cofactor23 = m.cofactor(2, 3);
        let cofactor32 = m.cofactor(3, 2);

        let expected_result = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(532.0, determinant);
        assert_fuzzy_eq!(-160.0, cofactor23);
        assert_fuzzy_eq!(-160.0 / 532.0, actual_result[3][2]);
        assert_fuzzy_eq!(105.0, cofactor32);
        assert_fuzzy_eq!(105.0 / 532.0, actual_result[2][3]);
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculating_the_inverse_of_another_4x4_matrix() {
        let m = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected_result = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_4x4_matrix() {
        let m = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let expected_result = Matrix::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let m2 = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let m3 = m1 * m2;

        let actual_result = m3 * m2.inverse();

        assert_fuzzy_eq!(actual_result, m1);
    }

    #[test]
    fn multiply_point_by_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(2.0, 1.0, 7.0);
        let actual = transform * point;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(-8.0, 7.0, 3.0);
        let actual = inverse * point;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let vector = Tuple::vector(-3.0, 4.0, 5.0);

        let expected = Tuple::vector(-3.0, 4.0, 5.0);
        let actual = transform * vector;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        let expected = Tuple::point(-8.0, 18.0, 32.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-8.0, 18.0, 32.0);
        let actual = transform * v;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn multiply_by_inverse_of_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let i = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-2.0, 2.0, 2.0);
        let actual = i * v;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(-2.0, 3.0, 4.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn rotate_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);

        let expected = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let actual = half_quarter * p;

        assert_fuzzy_eq!(expected, actual);

        let expected = Tuple::point(0.0, 0.0, 1.0);
        let actual = full_quarter * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        let expected = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
        let actual = inv * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn rotate_point_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        let expected = Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let actual = half_quarter * p;

        assert_fuzzy_eq!(expected, actual);

        let expected = Tuple::point(1.0, 0.0, 0.0);
        let actual = full_quarter * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn rotate_point_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        let expected = Tuple::point(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let actual = half_quarter * p;

        assert_fuzzy_eq!(expected, actual);

        let expected = Tuple::point(-1.0, 0.0, 0.0);
        let actual = full_quarter * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(5.0, 3.0, 4.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(6.0, 3.0, 4.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 5.0, 4.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 7.0, 4.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 3.0, 6.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 3.0, 7.0);
        let actual = transform * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn individual_transformation_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_fuzzy_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_fuzzy_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_fuzzy_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformation_are_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let t = Matrix::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        let expected = Tuple::point(15.0, 0.0, 7.0);
        let actual = t * p;

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn view_transformation_matrix_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);

        assert_fuzzy_eq!(Matrix::identity(), t);
    }

    #[test]
    fn view_transformation_matrix_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);

        assert_fuzzy_eq!(Matrix::scaling(-1.0, 1.0, -1.0), t);
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);

        assert_fuzzy_eq!(Matrix::translation(0.0, 0.0, -8.0), t);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let expected = Matrix::from([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        let t = Matrix::view_transform(from, to, up);
        assert_fuzzy_eq!(expected, t);
    }
}
