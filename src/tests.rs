use matrix::Matrix;
use matrixbuilder::MatrixBuilder;

//TEST DEFAULT IMPLEMENTATION

#[test]
fn test_set_get()
{
	let mut matrix: Matrix<usize> = Matrix::new(2, 2, 0).unwrap();
	matrix.set(0, 0, 1);
	matrix.set(0, 1, 2);
	matrix.set(1, 0, 3);
	matrix.set(1, 1, 4);
	assert!(matrix.get(0, 0).0 == 1);
	assert!(matrix.get(0, 1).0 == 2);
	assert!(matrix.get(1, 0).0 == 3);
	assert!(matrix.get(1, 1).0 == 4);
}

#[test]
fn test_size()
{
	let mut matrix: Matrix<usize> = Matrix::new(2, 5, 0).unwrap();
	assert_eq!(matrix.get_col_count(), 5);
	assert_eq!(matrix.get_row_count(), 2);
	assert_eq!(matrix.get_size(), (2, 5)); 
}

#[test]
fn test_swap()
{
	let mut matrix: Matrix<usize> = Matrix::new(2, 2, 0).unwrap();
	matrix.set(0, 0, 1);
	matrix.set(1, 1, 2);
	matrix.swap(0, 0, 1, 1);
	assert!(matrix.get(0, 0).0 == 2);
	assert!(matrix.get(1, 1).0 == 1);
}

#[test]
fn test_build()
{
	let mut mb = MatrixBuilder::new();
	mb.push_row(vec!(1,2));
	mb.push_row(vec!(3,4));
	let matrix = mb.build().unwrap();
	assert!(matrix.get(0, 0).0 == 1);
	assert!(matrix.get(0, 1).0 == 2);
	assert!(matrix.get(1, 0).0 == 3);
	assert!(matrix.get(1, 1).0 == 4);
}

#[test]
fn test_build_chain()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	assert!(matrix.get(0, 0).0 == 1);
	assert!(matrix.get(0, 1).0 == 2);
	assert!(matrix.get(1, 0).0 == 3);
	assert!(matrix.get(1, 1).0 == 4);
}

//TEST IMPL OF DISPLAY/DEBUG TRAIT

#[test]
fn test_display()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	println!("{}", matrix);
}

#[test]
fn test_debug()
{
	let matrix = MatrixBuilder::new().row(vec!(1,2)).row(vec!(3,4)).build().unwrap();
	println!("{:?}", matrix);
}