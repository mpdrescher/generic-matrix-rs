# generic-matrix-rs
This is basically a very basic 2D-Array with unknown size at compile-time.

I tried to keep the method names as self-explanatory as possible:

* `new(width, height, default) -> Matrix<T>`
* `width() -> usize`
* `height() -> usize`
* `bounds() -> (usize, usize)`: the same as (width(), height())
* `get(x, y) -> Option<&T>`
* `set(x, y, val) -> bool`
* `fields() -> &Vec<T>`
* `inbound(x, y) -> bool`

##Usage

Cargo.toml:

```
[dependencies]
matrix = {git = "https://github.com/mpdrescher/generic-matrix-rs"}
```
