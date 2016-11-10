///Holds all the functionality groups a matrix has

use matrix::Matrix;

//implementation needed
pub trait MatrixSlice<T>
{
	fn get_row(&self, row: usize) -> Vec<T>;
	fn get_col(&self, col: usize) -> Vec<T>;
	fn crop(self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> Matrix<T>;
	fn copy_crop(&self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> Matrix<T>;
	fn replace_area(&self, row_1: usize, col_1: usize, row_2: usize, col_2: usize, replacement: Matrix<T>); 
}





/*
transform
	rotate_90
	rotate_180
	rotate_270
	...wikipedia
*/