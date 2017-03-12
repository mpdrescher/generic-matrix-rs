# generic-matrix-rs
This is basically a very simple 2D-Array with unknown size at compile-time.

I tried to keep the method names as self-explanatory as possible.  
List of methods of `matrix::Matrix`:

* `fn new(width: usize, height: usize, default: T) -> Matrix<T>`
* `fn get(&self, x: usize, y: usize) -> Option<&T>`
* `fn set(&mut self, x: usize, y: usize, val: T) -> bool`
* `fn width(&self) -> usize`
* `fn height(&self) -> usize`
* `fn bounds(&self) -> (usize, usize)`: the same as `(width(), height())`
* `fn fields(&self) -> &Vec<T>`
* `fn inbound(&self, x: usize, y: usize) -> bool`

##Usage

Cargo.toml:

```
[dependencies]
matrix = {git = "https://github.com/mpdrescher/generic-matrix-rs"}
```
