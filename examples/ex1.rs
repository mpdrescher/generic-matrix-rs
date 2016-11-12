extern crate generic_matrix;

use generic_matrix::matrix::Matrix;
use generic_matrix::matrixbuilder::MatrixBuilder;

fn main()
{
	let mut matrix: Matrix<usize> = MatrixBuilder::new()
						.row(vec!(1, 2))
						.row(vec!(3, 4))
						.build()
						.unwrap();

	println!("field 1,1: {} \n", matrix.get(1, 1).unwrap());
	
	println!("changing field 1,1\n");
	let _ = matrix.set(1, 1, 5);
	
	println!("field 1,1: {} \n", matrix.get(1, 1).unwrap());

	println!("{}", matrix);
}