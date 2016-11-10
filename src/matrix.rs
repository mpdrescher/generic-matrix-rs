use matrixerror::MatrixError;

use std::fmt;

///Holds the matrix data in a Vec with the following structure:<br>
///(0,0);(0,1) .. (0,n);(1,0);(1,1) .. (1,n) .. (m,0);(m,1) .. (m,n)<br>
///The size of the matrix is m*n<br>
///m = row number<br>
///n = column number<br>
///<br>
///Be aware that the values of an index are NOT (x;y), but (row;col)<br>
#[derive(Clone, PartialEq)]
pub struct Matrix<T>
{
	columns: usize,
	rows: usize,
	fields: Vec<T>,
	default: T
}

impl<T> Matrix<T> where T: Clone
{
	///Constructs a new matrix with a given width + height<br>
	///and fills it with copies of a default value<br>
	///<br>
	///Returns None if either the width or the height of the matrix is zero
	pub fn new(rows: usize, cols: usize, default: T) -> Option<Matrix<T>>
	{
		if rows == 0 || cols == 0
		{
			return None;
		}
		let vecsize = rows*cols;
		let mut fields = Vec::with_capacity(vecsize);
		for _ in 0..vecsize
		{
			fields.push(default.clone());
		}
		Some(Matrix
		{
			columns: cols,
			rows: rows,
			fields: fields,
			default: default
		})
	}

	///Constructs a new matrix with a given width + height<br>
	///and fills it with data from a vector (data structure is described in Matrix struct doc)<br>
	///<br>
	///Returns None if:<br>
	///-vector size doesn't match rows * cols
	///-rows or cols is zero
	pub fn from_vec(vector: Vec<T>, rows: usize, cols: usize) -> Option<Matrix<T>>
	{
		let size = rows*cols;
		if vector.len() != size
		{
			return None;
		}
		if rows == 0 || cols == 0
		{
			return None;
		}
		let default = vector.get(0).unwrap().clone();
		Some(
		Matrix
		{
			columns: cols,
			rows: rows,
			fields: vector,
			default: default
		})
	}

	///Returns a reference to the value at the given position -> M(row, col)<br>
	///<br>
	///Errors:<br>
	///-IndexOOB
	pub fn get_ref(&self, row: usize, col: usize) -> (&T, MatrixError)
	{
		if !self.is_valid_index(row, col)
		{
			return (&self.default, MatrixError::IndexOOB);
		}
		let index = self.index_of(row, col);
		let result = self.fields.get(index).unwrap(); //should not happen
		(result, MatrixError::None)
	}

	///Returns a copy of the value at the given position -> M(row, col)<br>
	///<br>
	///Errors:<br>
	///-IndexOOB
	pub fn get(&self, row: usize, col: usize) -> (T, MatrixError)
	{
		let (result, error) = self.get_ref(row, col);
		(result.clone(), error)
	}

	///Sets the field at the given position (M(row, col))<br>
	///<br>
	///Errors:<br>
	///-IndexOOB
	pub fn set(&mut self, row: usize, col: usize, value: T) -> MatrixError
	{
		let (mutref, error) = self.get_mut(row, col);
		if error != MatrixError::None
		{
			return error;
		}
		match mutref
		{
			Some(mut v) => {
				*v = value;
			},
			//would only get called in case of OOB error, but OOB is handled in if error != MatrixError::None
			None => {} 
		};
		MatrixError::None
	}

	pub fn swap(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> MatrixError
	{
		let (temp_1, temp1_error) = self.get(row_1, col_1);
		let (temp_2, temp2_error) = self.get(row_2, col_2);
		if temp1_error != MatrixError::None
		{
			return temp1_error
		}
		if temp2_error != MatrixError::None
		{
			return temp2_error
		}
		self.set(row_1, col_1, temp_2);
		self.set(row_2, col_2, temp_1);
		MatrixError::None
	}

	///Returns the width/columns of the matrix
	pub fn get_col_count(&self) -> usize
	{
		self.columns
	}

	///Returns the height/rows of the matrix
	pub fn get_row_count(&self) -> usize
	{
		self.rows
	}

	///Returns a tuple representing the size of the matrix:<br>
	///(height/rows, width/columns)
	pub fn get_size(&self) -> (usize, usize)
	{
		(self.rows, self.columns)
	}

	///Checks if the given index is inside the matrix bounds
	pub fn is_valid_index(&self, row: usize, col: usize) -> bool
	{
		row < self.rows && col < self.columns //not necessary to check >= 0 -> usize, not isize
	}

	//like get(..), but not used because of set(..)
	fn get_mut(&mut self, row: usize, col: usize) -> (Option<&mut T>, MatrixError)
	{
		if !self.is_valid_index(row, col)
		{
			return (None, MatrixError::IndexOOB);
		}
		let index = self.index_of(row, col);
		let result = self.fields.get_mut(index).unwrap();
		(Some(result), MatrixError::None)
	}

	//projects the 2D indices to 1D index
	fn index_of(&self, row: usize, col: usize) -> usize
	{
		row * self.columns + col
	}
}

impl<T> fmt::Display for Matrix<T> where T: Clone + fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		let mut output = String::new();
		output.push('[');
		for row in 0..self.get_row_count()
		{
			output.push('[');
			for col in 0..self.get_col_count()
			{
				let val = self.get(row, col);
				if col != self.get_col_count() - 1
				{
					output.push_str(&format!("{}, ", val.0));
				}
				else 
				{
				    output.push_str(&format!("{}", val.0));
				}
			}
			output.push(']');
			if row != self.get_row_count() - 1
			{
				output.push('\n');
			}
		}
		output.push(']');
		write!(f, "{}", output)
	}
}

impl<T> fmt::Debug for Matrix<T> where T: Clone + fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		let mut output = String::new();
		output.push_str(&format!("{} x {}\n", self.get_row_count(), self.get_col_count()));
		output.push('[');
		for row in 0..self.get_row_count()
		{
			output.push('[');
			for col in 0..self.get_col_count()
			{
				let val = self.get(row, col);
				if col != self.get_col_count() - 1
				{
					output.push_str(&format!("{:?}, ", val.0));
				}
				else 
				{
				    output.push_str(&format!("{:?}", val.0));
				}
			}
			output.push(']');
			if row != self.get_row_count() - 1
			{
				output.push('\n');
			}
		}
		output.push(']');
		write!(f, "{}", output)
	}
}