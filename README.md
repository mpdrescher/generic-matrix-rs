# generic-matrix-rs
Implements a dynamically allocated, generic matrix structure in Rust  
  

This library can also be used as a 2D-Array stand-in,  
since regular Rust 2D Arrays can not be dynamically allocated.

##Current Features
* Build a matrix with a MatrixBuilder
* Generate a matrix from a vector
* Basics (get & set & swap & get size)
* Apply a closure to each element of the matrix
* Apply a closure to each element of two matrices
* Get row/column
* Get area
* Replace area
* Reshape
* Transpose
* Flip horizontally/vertically
* Debug/Display
* Transform into iterator
* Count occurences/list indices of an entry in the matrix