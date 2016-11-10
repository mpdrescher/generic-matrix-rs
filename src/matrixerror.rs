#[derive(PartialEq)]
///A list of errors that might occur while operating on the matrix
pub enum MatrixError
{
	///Indicates an attempt of accessing a value that does not lie within the bounds of the matrix
	IndexOOB,
	///Indicates that everything is fine
	None
}