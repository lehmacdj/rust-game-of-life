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
        let Frame { width: w, .. } = *self;
        w
    }

    /// the height of the frame
    pub fn height(&self) -> usize {
        let Frame { height: l, .. } = *self;
        l
    }

    /// the internal data array
    fn data(&self) -> &Vec<Vec<T>> {
        let Frame { data: ref d, .. } = *self;
        d
    }

    /// mutable reference to the internal data array
    fn data_mut(&mut self) -> &mut Vec<Vec<T>> {
        let Frame { data: ref mut d, .. } = *self;
        d
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

impl<T> Frame<T>
where T: Clone {
    /// return the next frame of the simulation advancing the simulation using
    /// a step function that computes the value for any cell given a certain
    /// board
    pub fn next_frame<F>(&self, step: F) -> Frame<T>
    where F: Fn(&Frame<T>, (usize, usize)) -> T {
        let mut data = self.data().clone();
        for (x, row) in self.data().iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                data[x][y] = step(&self, (x, y));
            }
        }

        Frame::<T> {
            data: data,
            width: self.width(),
            height: self.height(),
        }
    }
}
