use std::ops::{AddAssign, Mul};
use crate::matrix::Matrix;

pub fn multiply<T>(matrix_a: Matrix<T>, matrix_b: Matrix<T>) -> Result<Matrix<T>, String>
    where
        for<'a> &'a T: Mul<Output=T>,
        T: AddAssign<T> {


    if matrix_a.get_columns() != matrix_b.get_rows() {
        return Err(format!("A columns: {} and B rows: {} don't match!",
                           matrix_a.get_columns(), matrix_b.get_rows()));
    }

    let rows = matrix_a.get_rows();

    let columns = matrix_b.get_columns();

    let mut numbers: Vec<T> = Vec::with_capacity(rows * columns);

    let a_numbers = matrix_a.get_numbers();
    let b_numbers = matrix_b.get_numbers();

    let n = matrix_a.get_columns();

    for row in 0..rows {
        for column in 0..columns {
            let mut sum = &a_numbers[row * matrix_a.get_columns()] * &b_numbers[column];
            for k in 1..n {
                sum += &a_numbers[row * matrix_a.get_columns() + k] * &b_numbers[k * matrix_b.get_columns() + column];
            }
            numbers.push(sum);
        }
    }

    Ok(Matrix::new(rows, columns, numbers).unwrap())
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use crate::multiplication::multiply;

    #[test]
    fn multiplication_correct() {
        let matrix_a = Matrix::<i32>::from_vec(
            vec!["3", "2", "1 2", "3 4", "5 6"]).unwrap();

        let matrix_b = Matrix::<i32>::from_vec(
            vec!["2", "4", "7 8 9 10", "11 12 13 14"]).unwrap();

        let expected = Matrix::<i32>::from_vec(
            vec!["3", "4", "29 32 35 38", "65 72 79 86", "101 112 123 134"]).unwrap();

        let result = multiply(matrix_a, matrix_b).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_incorrect_matrix_dimensions() {
        let matrix_a = Matrix::<i32>::from_vec(
            vec!["3", "2", "1 2", "3 4", "5 6"]).unwrap();

        let matrix_b = Matrix::<i32>::from_vec(
            vec!["3", "4", "7 8 9 10", "11 12 13 14", "15 16 17 18"]).unwrap();

        let result = multiply(matrix_a, matrix_b);

        assert!(result.is_err());
    }


}
