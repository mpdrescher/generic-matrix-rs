#[derive(PartialEq, Debug)]
///A list of errors that might occur while operating on the matrix
pub enum MatrixError
{
	///Indicates that the matrix before and after the reshape would have different sizes
	ReshapeNotPossible,
	///Indicates that the replacement has not the size of the area that should be replaced
	ReplacementMismatch,
	///Indicates that while creating a matrix from a vector the vector doesn't have the right size
	InvalidVecSize,
	///Indicates a matrix size of 0
	InvalidSize,
	///Indicates an attempt of accessing a value that does not lie within the bounds of the matrix
	IndexOOB
}