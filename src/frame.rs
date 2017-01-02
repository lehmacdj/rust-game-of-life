/// Represents a frame of a simulation
/// This internal representation is not stable and should not be relied upon
#[derive(Debug, PartialEq)]
pub struct Frame<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Frame<T>
where T: Default + Clone {
    /// Creates an empty frame
    pub fn new(x: usize, y: usize) -> Frame<T> {
        let default = vec![T::default(); y];
        Frame::<T> {data: vec![default; x], width: x, height: y}
    }
}

/// Getters / setters for the data in the frame
impl<T> Frame<T> {
    /// the width of the frame
    pub fn width(&self) -> usize {
        self.width
    }

    /// the height of the frame
    pub fn height(&self) -> usize {
        self.height
    }

    /// the internal data array
    fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    /// mutable reference to the internal data array
    fn data_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.data
    }

    /// the data at (x, y)
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data()[x][y]
    }

    /// set the data at (x, y) to d
    pub fn set(&mut self, x: usize, y: usize, d: T) {
        self.data_mut()[x][y] = d;
    }
}

/// Represents a single square in the frame
#[derive(Debug, PartialEq)]
pub struct Square<'a, T>
where T: 'a {
    frame: &'a Frame<T>,
    point: (usize, usize),
}

/// Add x and y mod m
fn add_modulo(x: usize, y: isize, m: usize) -> usize {
    // y should not be greater than the modulo we are working with
    assert!((y.abs() as usize) < m);
    let base = (x % m) + m;
    let delta = if y.is_negative() { y + (m as isize) } else { y } as usize;
    (base + delta) % m
}

impl<'a, T> Square<'a, T>
where T: 'a {
    /// Return a point relative to the square
    pub fn get(&self, i: isize, j: isize) -> &T {
        let (x, y) = self.point;
        let width = self.frame.width();
        let height = self.frame.height();
        let (x, y) = (add_modulo(x, i, width), add_modulo(y, j, height));
        self.frame.get(x, y)
    }
}

impl<T> Frame<T>
where T: Clone {
    /// return the next frame of the simulation advancing the simulation using
    /// a step function that computes the value for any cell given a certain
    /// board
    pub fn next_frame<F>(&self, step: F) -> Frame<T>
    where F: Fn(Square<T>) -> T {
        let mut data = self.data().clone();
        for (x, row) in self.data().iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                let square = Square {
                    frame: &self,
                    point: (x, y),
                };
                data[x][y] = step(square);
            }
        }

        Frame {
            data: data,
            width: self.width(),
            height: self.height(),
        }
    }
}
