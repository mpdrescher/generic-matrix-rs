use matrix::Matrix;
use matrixbuilder::MatrixBuilder;
use matrixtraits::MatrixSlice;

//TEST DEFAULT IMPLEMENTATION

#[test]
fn test_set_get()
{
	let mut matrix: Matrix<usize> = Matrix::new(2, 2, 0).unwrap();
	let _ = matrix.set(0, 0, 1);
	let _ = matrix.set(0, 1, 2);
	let _ = matrix.set(1, 0, 3);
	let _ = matrix.set(1, 1, 4);
	assert!(matrix.get(0, 0).unwrap() == 1);
	assert!(matrix.get(0, 1).unwrap() == 2);
	assert!(matrix.get(1, 0).unwrap() == 3);
	assert!(matrix.get(1, 1).unwrap() == 4);
}

#[test]
fn test_size()
{
	let matrix: Matrix<usize> = Matrix::new(2, 5, 0).unwrap();
	assert_eq!(matrix.get_col_count(), 5);
	assert_eq!(matrix.get_row_count(), 2);
	assert_eq!(matrix.get_size(), (2, 5)); 
}

#[test]
fn test_swap()
{
	let mut matrix: Matrix<usize> = Matrix::new(2, 2, 0).unwrap();
	let _ = matrix.set(0, 0, 1);
	let _ = matrix.set(1, 1, 2);
	let _ = matrix.swap(0, 0, 1, 1);
	assert!(matrix.get(0, 0).unwrap() == 2);
	assert!(matrix.get(1, 1).unwrap() == 1);
}

#[test]
fn test_build()
{
	let mut mb = MatrixBuilder::new();
	mb.push_row(vec!(1,2));
	mb.push_row(vec!(3,4));
	let matrix = mb.build().unwrap();
	assert!(matrix.get(0, 0).unwrap() == 1);
	assert!(matrix.get(0, 1).unwrap() == 2);
	assert!(matrix.get(1, 0).unwrap() == 3);
	assert!(matrix.get(1, 1).unwrap() == 4);
}

#[test]
fn test_build_chain()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	assert!(matrix.get(0, 0).unwrap() == 1);
	assert!(matrix.get(0, 1).unwrap() == 2);
	assert!(matrix.get(1, 0).unwrap() == 3);
	assert!(matrix.get(1, 1).unwrap() == 4);
}

//TEST IMPL OF DISPLAY/DEBUG TRAIT

#[test]
fn test_display()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	println!("DISPLAY:\n{}", matrix);
}

#[test]
fn test_debug()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	println!("DISPLAY:\n{:?}", matrix);
}

//TEST IMPL OF MATRIXSLICE

#[test]
fn test_get_row()
{
	let matrix = MatrixBuilder::new()
						.row(vec!(1,2))
						.row(vec!(3,4))
						.build()
						.unwrap();
	assert!(matrix.get_row(0).unwrap() == vec!(1,2));
	assert!(matrix.get_row(1).unwrap() == vec!(3,4));
}

#[test]
fn test_get_col()
{
	let matrix = MatrixBuilder::new()
						.row(vec!(1,2))
						.row(vec!(3,4))
						.build()
						.unwrap();
	assert!(matrix.get_col(0).unwrap() == vec!(1,3));
	assert!(matrix.get_col(1).unwrap() == vec!(2,4));
}

#[test]
fn test_get_area()
{
	let matrix = MatrixBuilder::new()
						.row(vec!(1,2,3))
						.row(vec!(4,5,6))
						.row(vec!(7,8,9))
						.build()
						.unwrap();
	let matrix_2 = MatrixBuilder::new()
						.row(vec!(5,6))
						.row(vec!(8,9))
						.build()
						.unwrap();
	//println!("AREA:\n{}", matrix.get_area(1,1,2,2).unwrap());
	assert!(matrix.get_area(1, 1, 2, 2).unwrap() == matrix_2);
}

#[test]
fn test_replace_area()
{
	let mut matrix = MatrixBuilder::new()
						.row(vec!(1,2,3))
						.row(vec!(4,5,6))
						.row(vec!(7,8,9))
						.build()
						.unwrap();
	let matrix_2 = MatrixBuilder::new()
						.row(vec!(5,6,3))
						.row(vec!(8,9,6))
						.row(vec!(7,8,9))
						.build()
						.unwrap();
	let rep = matrix.get_area(1,1,2,2).unwrap();
	matrix.replace_area(0,0,1,1, rep).unwrap();
	//println!("REPLACE:\n{} \n{}", matrix, matrix_2);
	assert!(matrix == matrix_2);

}