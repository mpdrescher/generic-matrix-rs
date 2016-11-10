use matrix::Matrix;

///Struct used for creating a prefilled matrix
pub struct MatrixBuilder<T>
{
	rows: usize,
	columns: usize,
	buffer: Vec<Vec<T>>
}

impl<T> MatrixBuilder<T> where T: Clone
{
	///Creates a new MatrixBuilder
	pub fn new() -> MatrixBuilder<T>
	{
		MatrixBuilder
		{
			rows: 0,
			columns: 0,
			buffer: Vec::new()
		}
	}

	///Appends a row to the matrix
	pub fn push_row(&mut self, row: Vec<T>)
	{
		self.columns = row.len();
		self.rows += 1;
		self.buffer.push(row);
	}

	///Appends a row to the matrix, but returns the MatrixBuilder to<br>
	///enable chained construction:<br>
	///<br>
	///e.g: let matrix = MatrixBuilder::new().row(..).row(..).build().unwrap();
	pub fn row(mut self, row: Vec<T>) -> MatrixBuilder<T>
	{
		self.columns = row.len();
		self.rows += 1;
		self.buffer.push(row);
		self
	}

	///Creates a matrix from the row buffer
	pub fn build(self) -> Option<Matrix<T>>
	{
		let mut buildvec: Vec<T> = Vec::new();
		for mut vector in self.buffer
		{
			buildvec.append(&mut vector);	
		}
		Matrix::from_vec(buildvec, self.rows, self.columns)
	}
}