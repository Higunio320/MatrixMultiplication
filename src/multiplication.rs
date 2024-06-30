use std::ops::{AddAssign, Mul};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::matrix::Matrix;

pub fn multiply<T>(matrix_a: &Matrix<T>, matrix_b: &Matrix<T>, num_of_threads: usize) -> Result<Matrix<T>, String>
    where
        for<'a> &'a T: Mul<Output=T>,
        T: AddAssign<T> + Sync + Send + 'static {

    if matrix_a.get_columns() != matrix_b.get_rows() {
        return Err(format!("A columns: {} and B rows: {} don't match!",
                           matrix_a.get_columns(), matrix_b.get_rows()));
    }

    let rows = matrix_a.get_rows();

    if num_of_threads > rows {
        return Err(format!(
            "Num of threads: {num_of_threads} cannot be higher than Matrix A rows: {rows}"))
    }

    if num_of_threads == 0 {
        return Err(format!(
            "Num of threads: {num_of_threads} must be higher than 0"))
    }

    let columns = matrix_b.get_columns();

    let n = matrix_a.get_columns();

    let a_numbers = matrix_a.get_numbers();
    let b_numbers = matrix_b.get_numbers();

    let rows_for_threads = generate_indexes_for_threads(num_of_threads, rows);

    let results_from_threads: Arc<Mutex<Vec<Vec<T>>>> = Arc::new(Mutex::new(Vec::with_capacity(num_of_threads)));
    for _ in 0..num_of_threads {
        results_from_threads.lock().unwrap().push(vec![])
    }

    let mut handles = Vec::with_capacity(num_of_threads);

    for i in 0..num_of_threads {
        let a_numbers = Arc::clone(&a_numbers);
        let b_numbers = Arc::clone(&b_numbers);

        let start_row = rows_for_threads[i];
        let end_row = rows_for_threads[i+1];

        let results_from_threads = Arc::clone(&results_from_threads);


        let handle = thread::spawn(move || {
            let mut result = Vec::with_capacity((end_row - start_row) * columns);

            for row in start_row..end_row {
                for column in 0..columns {
                    let mut sum = &a_numbers[row * n] * &b_numbers[column];
                    for k in 1..n {
                        sum += &a_numbers[row * n + k] * &b_numbers[k * columns + column];
                    }
                    result.push(sum);
                }
            }

            let mut results = results_from_threads.lock().expect(
                format!("Error acquiring mutex lock for thread {i}").as_str());
            results[i] = result;
        });

        handles.push(handle);
    }

    for (thread_num,handle) in handles.into_iter().enumerate() {
        if let Err(err) = handle.join() {
            return Err(format!("Error joining thread {thread_num}, error:\n{err:?}"))
        }
    }

    if let Ok(mutex) = Arc::try_unwrap(results_from_threads) {
        if let Ok(vector) = mutex.into_inner() {
            let vector: Vec<T> = vector.into_iter().flat_map(|vec| vec).collect();
            Ok(Matrix::new(rows, columns, vector).unwrap())
        } else {
            Err(String::from("Error acquiring mutex in main thread"))
        }
    } else {
        Err(String::from("Error unwrapping results in main thread"))
    }
}

fn generate_indexes_for_threads(num_of_threads: usize, rows: usize) -> Vec<usize> {
    let rows_per_thread = rows / num_of_threads;
    let mut rest: usize = rows % num_of_threads;

    let mut rows_for_threads = Vec::with_capacity(num_of_threads+1);

    let mut current = 0;

    rows_for_threads.push(0);

    for _ in 0..num_of_threads {
        current += rows_per_thread;
        if rest > 0 {
            current += 1;
            rest -= 1;
        }
        rows_for_threads.push(current);
    }

    rows_for_threads
}

pub fn run(config: Config) -> Result<(), String> {
    let matrix_a = Matrix::<f64>::from_file(config.matrix_a_file_name.as_str())?;
    let matrix_b = Matrix::<f64>::from_file(config.matrix_b_file_name.as_str())?;

    let matrix_c = multiply(&matrix_a, &matrix_b, 10)?;

    return matrix_c.to_file(config.matrix_c_file_name.as_str())
}

pub struct Config {
    matrix_a_file_name: String,
    matrix_b_file_name: String,
    matrix_c_file_name: String
}

impl Config {
    pub fn from_iter(mut iterator: impl Iterator<Item=String>) -> Result<Config, String> {
        iterator.next();

        let matrix_a_file_name = match iterator.next() {
            Some(file_name) => file_name,
            None => return Err(String::from("Missing Matrix A file name"))
        };

        let matrix_b_file_name = match iterator.next() {
            Some(file_name) => file_name,
            None => return Err(String::from("Missing Matrix B file name"))
        };

        let matrix_c_file_name = match iterator.next() {
            Some(file_name) => file_name,
            None => return Err(String::from("Missing Matrix C file name"))
        };

        Ok(Config{ matrix_a_file_name, matrix_b_file_name, matrix_c_file_name })
    }
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

        let result = multiply(&matrix_a, &matrix_b, 1).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_incorrect_matrix_dimensions() {
        let matrix_a = Matrix::<i32>::from_vec(
            vec!["3", "2", "1 2", "3 4", "5 6"]).unwrap();

        let matrix_b = Matrix::<i32>::from_vec(
            vec!["3", "4", "7 8 9 10", "11 12 13 14", "15 16 17 18"]).unwrap();

        let result = multiply(&matrix_a, &matrix_b, 1);

        assert!(result.is_err());
    }

    #[test]
    fn parallel_multiplication_correct() {
        let matrix_a = Matrix::<i32>::from_vec(
            vec!["3", "2", "1 2", "3 4", "5 6"]).unwrap();

        let matrix_b = Matrix::<i32>::from_vec(
            vec!["2", "4", "7 8 9 10", "11 12 13 14"]).unwrap();

        let expected = Matrix::<i32>::from_vec(
            vec!["3", "4", "29 32 35 38", "65 72 79 86", "101 112 123 134"]).unwrap();

        let max_threads: usize = 3;

        for num_of_threads in 1..max_threads {
            let result = multiply(&matrix_a, &matrix_b, num_of_threads).unwrap();

            assert_eq!(result, expected);
        }
    }

}
