use std::fmt::format;
use std::fs;
use std::ops::Mul;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    rows: usize,
    columns: usize,
    numbers: Vec<T>
}

impl<T> Matrix<T> {
    pub fn get_rows(&self) -> usize {
        return self.rows;
    }

    pub fn get_columns(&self) -> usize {
        return self.columns;
    }

    pub fn get_numbers(&self) -> &Vec<T> {
        return &self.numbers;
    }

    pub fn new(rows: usize, columns: usize, numbers: Vec<T>) -> Result<Matrix<T>, String> {
        if numbers.len() != rows * columns {
            return Err(format!("Numbers length: {} doesn't match rows * columns: {} * {} = {}",
                               numbers.len(), rows, columns, rows * columns))
        }

        Ok(Matrix { rows, columns, numbers })
    }
}

impl<T: FromStr> Matrix<T> {
    pub fn from_file(file_name: &str) -> Result<Matrix<T>, String> {
        let contents = match fs::read_to_string(file_name) {
            Ok(contents) => contents,
            Err(error) => return Err(format!("Couldn't open file {}\nerror: {}", file_name, error))
        };

        return Self::from_iterator(contents.lines());
    }

    pub fn from_vec(vector: Vec<&str>) -> Result<Matrix<T>, String> {
        return Self::from_iterator(vector.iter());
    }

    fn from_iterator<'a>(mut iterator: impl Iterator<Item=&'a str>) -> Result<Matrix<T>, String> {
        let rows = match iterator.next() {
            Some(rows) => {match rows.trim().parse::<usize>() {
                Ok(parsed) => parsed,
                Err(_) => return Err(format!("Couldn't parse '{}' as rows num", rows))
            }}
            None => return Err(String::from("File is empty!"))
        };

        let columns = match iterator.next() {
            Some(columns) => {match columns.trim().parse::<usize>() {
                Ok(parsed) => parsed,
                Err(_) => return Err(format!("Couldn't parse '{}' as columns num", columns))
            }}
            None => return Err(String::from("File doesn't have columns row"))
        };

        let mut numbers = Vec::with_capacity(rows * columns);

        for i in 0..rows {
            let row = match iterator.next() {
                Some(row) => row.split_whitespace(),
                None => return Err(format!("Not enough rows: {i}"))
            };

            let row: Result<Vec<T>, _> = row
                .into_iter()
                .map(|num| num.parse::<T>())
                .collect();

            match row {
                Ok(mut parsed) if parsed.len() == columns => numbers.append(&mut parsed),
                Ok(parsed) => return Err(format!("Row {} length: {} doesn't match columns: {}",
                                                 i, parsed.len(), columns)),
                Err(_) => return Err(format!("Error parsing {} row", i)),
            }
        }

        Ok(Matrix { rows, columns, numbers })
    }
}

#[cfg(test)]
mod matrix_test {
    use crate::matrix::Matrix;

    #[test]
    fn iter_read_correct_ints() {
        let contents = vec!["3", "2", " 1 2", "3 4", "5 6"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        match matrix {
            Ok(matrix) => assert_eq!(matrix,
                                     Matrix{ rows: 3, columns: 2, numbers: vec![1, 2, 3, 4, 5, 6] }),
            Err(_) => assert!(false)
        }
    }


    #[test]
    fn iter_read_correct_floats() {
        let contents = vec!["3", "2", "1.2 2.567", "3.45 4.2", "5.0 6.0"];

        let matrix = Matrix::<f32>::from_iterator(contents.into_iter());

        match matrix {
            Ok(matrix) => assert_eq!(matrix,
                                     Matrix{ rows: 3, columns: 2, numbers: vec![1.2, 2.567, 3.45, 4.2, 5.0, 6.0] }),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn iter_read_floats_as_ints() {
        let contents = vec!["3", "2", "1.2 2.567", "3.45 4.2", "5.0 6.0"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_blank() {
        let contents = vec![""];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_no_columns() {
        let contents = vec!["2"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_no_matrix() {
        let contents = vec!["3", "2"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_wrong_rows() {
        let contents = vec!["a"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_wrong_columns() {
        let contents = vec!["2", "a"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_incorrect_rows_in_matrix_data() {
        let contents = vec!["3", "2", "1 2", "3 4"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_incorrect_columns_in_matrix_data() {
        let contents = vec!["3", "2", "1 2 3", "4 5 6", "7 8 9"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }

    #[test]
    fn iter_read_wrong_data_in_matrix_data() {
        let contents = vec!["3", "2", "1 2 3", "a 5 6", "7 8 9"];

        let matrix = Matrix::<i32>::from_iterator(contents.into_iter());

        assert!(matrix.is_err());
    }
}