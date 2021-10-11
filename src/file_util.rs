use csv::ReaderBuilder;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use std::fs::File;
use log::debug;

// read 2d vector from file
pub fn load_file_2d_vec(filepath: String, rows: usize, cols: usize) -> Vec<Vec<i64>> {
    debug!("load_file_2d_vec");
    // Read an array back from the file
    let file = File::open(filepath).unwrap(); 
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<i64> = reader.deserialize_array2((rows, cols)).unwrap();
    let mut res = vec![];
    for i in 0..array_read.shape()[0] {
        res.push(array_read.row(i).to_vec());
    }
    res
}

// read 2d array from file
pub fn load_file_2d_array(filepath: String, rows: usize, cols: usize) -> Array2<i64> {
    debug!("load_file_2d_array");
    // Read an array back from the file
    let file = File::open(filepath).unwrap();
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<i64> = reader.deserialize_array2((rows, cols)).unwrap();
    array_read
}