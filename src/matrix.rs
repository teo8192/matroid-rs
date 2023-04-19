use std::{
    fmt::Display,
    ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

pub trait Matrix<E>: Index<(usize, usize), Output = E> + IndexMut<(usize, usize)> + Sized
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    /// create a new matrix of the given size, filled with zeros
    fn new(rows: usize, cols: usize) -> Self;

    /// the number of rows in the matrix
    fn num_rows(&self) -> usize;

    /// the number of columns in the matrix
    fn num_cols(&self) -> usize;

    /// create a clone of the matrix
    fn clone(&self) -> Self {
        let mut a = Self::new(self.num_rows(), self.num_cols());

        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                a[(i, j)] = self[(i, j)].clone();
            }
        }

        a
    }

    /// create a matrix from the specified columns
    fn subset_matrix(&self, columns: &[usize]) -> Self {
        let mut cols = Vec::from(columns);
        cols.sort();
        cols.dedup();
        let mut a = Self::new(self.num_rows(), cols.len());

        for i in 0..self.num_rows() {
            for (j, &c) in cols.iter().enumerate() {
                a[(i, j)] = self[(i, c)].clone();
            }
        }

        a
    }

    /// Swap two columns in the matrix
    fn swap_cols(&mut self, a: usize, b: usize) {
        // for each place in the columns, swap the elements
        for i in 0..self.num_rows() {
            let temp = self[(i, a)].clone();
            self[(i, a)] = self[(i, b)].clone();
            self[(i, b)] = temp;
        }
    }

    /// Swap two rows in the matrix
    fn swap_rows(&mut self, a: usize, b: usize) {
        // for each place in the rows, swap the elements
        for i in 0..self.num_cols() {
            let temp = self[(a, i)].clone();
            self[(a, i)] = self[(b, i)].clone();
            self[(b, i)] = temp;
        }
    }

    // Multiply a row by a scalar
    fn multiply_row(&mut self, row: usize, scalar: E) {
        for i in 0..self.num_cols() {
            self[(row, i)] = self[(row, i)].clone() * scalar.clone();
        }
    }

    // Divide a row by a scalar
    fn divide_row(&mut self, row: usize, scalar: E) {
        for i in 0..self.num_cols() {
            self[(row, i)] = self[(row, i)].clone() / scalar.clone();
        }
    }

    /// row a += k * b
    fn add_row_to_row(&mut self, a: usize, b: usize, k: E) {
        for i in 0..self.num_cols() {
            self[(a, i)] = self[(a, i)].clone() + k.clone() * self[(b, i)].clone();
        }
    }

    /// Turn the matrix into row echelon form
    fn gauss_jordan(&mut self) {
        let mut i = 0;
        let mut j = 0;
        while i < self.num_rows() && j < self.num_cols() {
            // find the first non-zero element in the column
            let mut k = i;
            while k < self.num_rows() && self[(k, j)] == E::from(0u8) {
                k += 1;
            }
            if k == self.num_rows() {
                j += 1;
                continue;
            }
            // if the element is not zero, swap the rows
            if k != i {
                self.swap_rows(i, k);
            }
            // divide the row by the element
            self.divide_row(i, self[(i, j)].clone());

            // make all other elements in the column 0
            for k in 0..self.num_rows() {
                if k != i {
                    self.add_row_to_row(k, i, -self[(k, j)].clone());
                }
            }
            i += 1;
            j += 1;
        }
    }

    /// Calculate the rank of the matrix (the number of dimensions in the row-space)
    /// The matrix HAS to be in row-echelon form
    fn rank(&self) -> usize {
        let mut r = 0;
        for i in 0..self.num_rows() {
            let mut zero = true;
            for j in 0..self.num_cols() {
                if self[(i, j)] != E::from(0u8) {
                    zero = false;
                    break;
                }
            }
            if zero {
                break;
            }
            r += 1;
        }
        r
    }
}

#[derive(PartialEq, Eq)]
pub struct DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    rows: usize,
    cols: usize,
    data: Box<[E]>,
}

impl<E> Index<(usize, usize)> for DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    type Output = E;

    fn index(&self, (i, j): (usize, usize)) -> &E {
        &self.data[i * self.cols + j]
    }
}

impl<E> IndexMut<(usize, usize)> for DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut E {
        &mut self.data[i * self.cols + j]
    }
}

impl<E> Matrix<E> for DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    fn new(rows: usize, cols: usize) -> Self {
        DynMatrix {
            rows,
            cols,
            data: vec![E::from(0u8); rows * cols].into_boxed_slice(),
        }
    }

    fn num_rows(&self) -> usize {
        self.rows
    }

    fn num_cols(&self) -> usize {
        self.cols
    }
}

impl<E> DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
{
    #[allow(unused)]
    pub fn from_columns(columns: &[&[E]]) -> Option<Self> {
        let ncols = columns.len();
        let rows = columns[0].len();
        let mut a = Self::new(rows, ncols);
        for j in 0..ncols {
            if columns[j].len() != rows {
                return None;
            }
            for i in 0..rows {
                a[(i, j)] = columns[j][i].clone();
            }
        }
        Some(a)
    }

    #[allow(unused)]
    pub fn from_rows(rows: &[&[E]]) -> Option<Self> {
        let nrows = rows.len();
        let cols = rows[0].len();
        let mut a = Self::new(nrows, cols);
        for i in 0..nrows {
            if rows[i].len() != cols {
                return None;
            }
            for j in 0..cols {
                a[(i, j)] = rows[i][j].clone();
            }
        }
        Some(a)
    }

    /// Create a new matrix that is the same as this one, but without rows containing all zeros
    pub fn remove_zero_rows(&self) -> Self {
        let mut rows = Vec::new();

        for i in 0..self.num_rows() {
            let mut zero = true;
            for j in 0..self.num_cols() {
                if self[(i, j)] != E::from(0u8) {
                    zero = false;
                    break;
                }
            }
            if !zero {
                let mut row = Vec::with_capacity(self.num_cols());
                for j in 0..self.num_cols() {
                    row.push(self[(i, j)].clone());
                }
                rows.push(row);
            }
        }

        let mut matrix = Self::new(rows.len(), self.num_cols());
        for i in 0..rows.len() {
            for j in 0..self.num_cols() {
                matrix[(i, j)] = rows[i][j].clone();
            }
        }

        matrix
    }
}

// {{{ Display stuff

impl<E> Display for DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
        + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                write!(f, "{} ", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<E> std::fmt::Debug for DynMatrix<E>
where
    E: Clone
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq
        + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "rows: {}", self.rows)?;
        writeln!(f, "cols: {}", self.cols)?;
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                write!(f, "{:?} ", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_jordan() {
        let mut a =
            DynMatrix::from_columns(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]])
                .unwrap();
        println!("{}", a);
        a.gauss_jordan();
        println!("{}", a);
        assert_eq!(
            a,
            DynMatrix::from_rows(&[&[1.0, 0.0, -1.0], &[0.0, 1.0, 2.0], &[0.0, 0.0, 0.0]]).unwrap()
        );
    }

    #[test]
    fn rank1() {
        let mut a =
            DynMatrix::from_columns(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]])
                .unwrap();
        a.gauss_jordan();

        assert!(a.rank() == 2);
    }

    #[test]
    fn rank2() {
        let mut a = DynMatrix::from_columns(&[
            &[0.0, 0.0, 1.0],
            &[0.0, 1.0, 0.0],
            &[0.0, 1.0, 1.0],
            &[1.0, 0.0, 0.0],
            &[1.0, 0.0, 1.0],
            &[1.0, 1.0, 0.0],
            &[1.0, 1.0, 1.0],
        ])
        .unwrap();
        println!("{}", a);
        a.gauss_jordan();

        assert!(a.rank() == 3);
    }
}
