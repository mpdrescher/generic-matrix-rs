use matrixerror::MatrixError;
use matrixtraits::MatrixSlice;

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
}

impl<T> Matrix<T> where T: Clone
{
	///Constructs a new matrix with a given width + height<br>
	///and fills it with copies of a default value<br>
	///<br>
	///Possible errors:<br>
	///-IndexOOB
	pub fn new(rows: usize, cols: usize, default: T) -> Result<Matrix<T>, MatrixError>
	{
		if rows == 0 || cols == 0
		{
			return Err(MatrixError::IndexOOB);
		}
		let vecsize = rows*cols;
		let mut fields = Vec::with_capacity(vecsize);
		for _ in 0..vecsize
		{
			fields.push(default.clone());
		}
		Ok(Matrix
		{
			columns: cols,
			rows: rows,
			fields: fields,
		})
	}

	///Constructs a new matrix with a given width + height<br>
	///and fills it with data from a vector (data structure is described in Matrix struct doc)<br>
	///<br>
	///Possible errors:<br>
	///-InvalidVecSize
	///-InvalidSize
	pub fn from_vec(vector: Vec<T>, rows: usize, cols: usize) -> Result<Matrix<T>, MatrixError>
	{
		let size = rows*cols;
		if vector.len() != size
		{
			return Err(MatrixError::InvalidVecSize);
		}
		if rows == 0 || cols == 0
		{
			return Err(MatrixError::InvalidSize);
		}
		Ok(
		Matrix
		{
			columns: cols,
			rows: rows,
			fields: vector,
		})
	}

	///Returns a reference to the value at the given position -> M(row, col)<br>
	///<br>
	///Possible errors: <br>
	///-IndexOOB
	pub fn get_ref(&self, row: usize, col: usize) -> Result<&T, MatrixError>
	{
		if !self.is_valid_index(row, col)
		{
			return Err(MatrixError::IndexOOB)
		}
		let index = self.index_of(row, col);
		let result = self.fields.get(index).unwrap(); //should not happen
		Ok(result)
	}

	///Returns a copy of the value at the given position -> M(row, col)<br>
	///<br>
	///Possible errors: <br>
	///-IndexOOB
	pub fn get(&self, row: usize, col: usize) -> Result<T, MatrixError>
	{
		let reference = try!(self.get_ref(row, col));
		Ok(reference.clone())
	}

	///Sets the field at the given position (M(row, col))<br>
	///<br>
	///Possible errors: <br>
	///-IndexOOB
	pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), MatrixError>
	{
		let mutref = try!(self.get_mut(row, col));
		*mutref = value;
		Ok(())
	}

	///Swaps values of two fields<br>
	///<br>
	///Possible errors:<br>
	///-IndexOOB
	pub fn swap(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> Result<(), MatrixError>
	{
		let temp_1 = try!(self.get(row_1, col_1));
		let temp_2 = try!(self.get(row_2, col_2));
		try!(self.set(row_1, col_1, temp_2));
		try!(self.set(row_2, col_2, temp_1));
		Ok(())
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
	//
	//Possible errors:
	//-IndexOOB
	fn get_mut(&mut self, row: usize, col: usize) -> Result<&mut T, MatrixError>
	{
		if !self.is_valid_index(row, col)
		{
			return Err(MatrixError::IndexOOB);
		}
		let index = self.index_of(row, col);
		let result = self.fields.get_mut(index).unwrap();
		Ok(result)
	}

	//projects the 2D indices to 1D index
	fn index_of(&self, row: usize, col: usize) -> usize
	{
		row * self.columns + col
	}
}

impl<T> MatrixSlice<T> for Matrix<T> where T: Clone
{
	///Returns a row of the matrix as a vector<br>
	///<br>
	///Possible errors:<br>
	///-IndexOOB
	fn get_row(&self, row: usize) -> Result<Vec<T>, MatrixError>
	{
		if row >= self.get_row_count()
		{
			return Err(MatrixError::IndexOOB);
		}
		let mut result = Vec::new();
		for col in 0..self.get_col_count()
		{
			result.push(try!(self.get(row, col)));
		}
		Ok(result)
	}

	///Returns a column of the matrix as a vector<br>
	///<br>
	///Possible errors: <br>
	///-IndexOOB
	fn get_col(&self, col: usize) -> Result<Vec<T>, MatrixError>
	{
		if col >= self.get_col_count()
		{
			return Err(MatrixError::IndexOOB);
		}
		let mut result = Vec::new();
		for row in 0..self.get_row_count()
		{
			result.push(try!(self.get(row, col)));
		}
		Ok(result)
	}
	
	///Returns a subarea of the matrix<br>
	///The area is defined by:<br>
	///Top left corner: (row_1, col_1)<br>
	///Bottom right corner: (row_2, col_2)<br<
	///<br>
	///Possible errors: <br>
	///-IndexOOB<br>
	///-InvalidVecSize (should not happen)<br>
	///-InvalidSize (should not happen)
	fn get_area(&self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) -> Result<Matrix<T>, MatrixError>
	{
		let row_2_post = row_2 + 1;
		let col_2_post = col_2 + 1;
		let new_rows = row_2_post - row_1;
		let new_cols = col_2_post - col_1;
		let mut val_vec = Vec::new();
		for row in row_1..row_2_post
		{
			for col in col_1..col_2_post
			{
				val_vec.push(try!(self.get(row, col)));
			}
		}
		let new_matrix: Matrix<T> = try!(Matrix::from_vec(val_vec, new_rows, new_cols));
		Ok(new_matrix)
	}

	///Inserts a matrix into self<br>
	///The insertion area is defined by:<br>
	///Top left corner: (row_1, col_1)<br>
	///Bottom right corner: (row_2, col_2)<br<
	///<br>
	///Possible errors: <br>
	///-IndexOOB<br>
	///-ReplacementMismatch
	fn replace_area(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize, replacement: Matrix<T>) -> Result<(), MatrixError>
	{
		let row_2_post = row_2 + 1;
		let col_2_post = col_2 + 1;
		let new_rows = row_2_post - row_1;
		let new_cols = col_2_post - col_1;
		if new_rows != replacement.get_row_count() || new_cols != replacement.get_col_count()
		{
			return Err(MatrixError::ReplacementMismatch);
		}
		let mut rep_row = 0;
		let mut rep_col = 0;
		for row in row_1..row_2_post
		{
			for col in col_1..col_2_post
			{
				let val = try!(replacement.get(rep_row, rep_col));
				try!(self.set(row, col, val));
				rep_col += 1;
			}
			rep_col = 0;
			rep_row += 1;
		}
		Ok(())
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
				let val = self.get(row, col).unwrap();
				if col != self.get_col_count() - 1
				{
					output.push_str(&format!("{}, ", val));
				}
				else 
				{
				    output.push_str(&format!("{}", val));
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
				let val = self.get(row, col).unwrap();
				if col != self.get_col_count() - 1
				{
					output.push_str(&format!("{:?}, ", val));
				}
				else 
				{
				    output.push_str(&format!("{:?}", val));
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