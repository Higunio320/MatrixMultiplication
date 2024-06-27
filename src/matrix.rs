use std::fs;
use std::ops::Mul;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    rows: usize,
    columns: usize,
    numbers: Vec<T>
}

impl<T: Mul<Output=T> + FromStr> Matrix<T> {
    pub fn from_file(file_name: &str) -> Result<Matrix<T>, String> {
        let contents = match fs::read_to_string(file_name) {
            Ok(contents) => contents,
            Err(error) => return Err(format!("Couldn't open file {}\nerror: {}", file_name, error))
        };

        return Self::from_iterator(contents.lines());
    }

    fn from_iterator<'a>(mut iterator: impl Iterator<Item=&'a str>) -> Result<Matrix<T>, String> {
        let rows = match iterator.next() {
            Some(rows) => {match rows.parse::<usize>() {
                Ok(parsed) => parsed,
                Err(_) => return Err(format!("Couldn't parse '{}' as rows", rows))
            }}
            None => return Err(String::from("File is empty!"))
        };

        let columns = match iterator.next() {
            Some(columns) => {match columns.parse::<usize>() {
                Ok(parsed) => parsed,
                Err(_) => return Err(format!("Couldn't parse '{}' as columns", columns))
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
                _ => return Err(format!("Error parsing {} row", i))
            }
        }

        Ok(Matrix { rows, columns, numbers })
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    fn correct_read_from_iter() {
        let content = vec!["3", "2", "1 2", "1 3", "2 5"];

        let matrix = Matrix::from_iterator(content.into_iter()).unwrap();

        assert_eq!(matrix, Matrix {rows: 3, columns: 2, numbers: vec![1, 2, 1, 3, 2, 5]});
    }

    fn incorrect_read_from_iter() {
        let content = vec!["3", "2", "1 2 3", "4 5 6", "7 8 9 "];

        let matrix = Matrix::<i32>::from_iterator(content.into_iter());

        assert!(matrix.is_err());
    }
}