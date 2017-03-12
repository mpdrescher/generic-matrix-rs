//because I want a simple Matrix struct with common method names that just works
pub struct Matrix<T> {
    fields: Vec<T>,
    width: usize,
    height: usize
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Matrix<T>{
        let mut vec = Vec::with_capacity(width*height);
        for _ in 0..width*height {
            vec.push(default.clone());
        }
        Matrix {
            fields: vec,
            width: width,
            height: height
        }
    }

    pub fn bounds(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if !self.check_bounds(x,y) {
            return None;
        }
        self.fields.get(self.map_coord(x,y))
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) -> bool {
        if !self.check_bounds(x,y) {
            return false;
        }
        let index = self.map_coord(x,y);
        match self.fields.get_mut(index) {
            Some(v) => *v = val,
            None => return false
        }
        true
    }

    pub fn fields(&self) -> &Vec<T> {
        &self.fields
    }

    fn map_coord(&self, x: usize, y: usize) -> usize {
        y*self.width + x
    }

    pub fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

#[cfg(test)]
mod tests {
    use ::Matrix;

    #[test]
    fn test() {
        let mut mat = Matrix::new(3,2,0);
        assert_eq!(mat.bounds(), (3,2));
        assert_eq!(mat.width(), 3);
        assert_eq!(mat.height(), 2);
        assert_eq!(mat.get(0,0).unwrap().clone(), 0);
        assert_eq!(mat.get(2,2), None);
        assert!(mat.set(1,1,5));
        assert_eq!(mat.get(1,1).unwrap().clone(), 5);
    }
}
