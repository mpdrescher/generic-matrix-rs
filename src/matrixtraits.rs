///Holds all the functionality groups a matrix has

use matrix::Matrix;
use matrixerror::MatrixError;

//implementation needed
pub trait MatrixSlice<T>
{
	fn get_row(&self, row: usize) -> Result<Vec<T>, MatrixError>;
	fn get_col(&self, col: usize) -> Result<Vec<T>, MatrixError>;
	fn get_area(&self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> Result<Matrix<T>, MatrixError>;
	fn replace_area(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize, replacement: Matrix<T>) -> Result<(), MatrixError>;
}





/*
transform
	rotate_90
	rotate_180
	rotate_270
	...wikipedia
*/