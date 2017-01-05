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

    /// get a mutable reference to the data at (x, y)
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data_mut()[x][y]
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

    pub fn coordinate(&self) -> (usize, usize) {
        self.point
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

/// An iterator over a Frame
#[derive(Debug, Clone, PartialEq)]
pub struct FrameIterator<'a, T>
where T: 'a {
    frame: &'a Frame<T>,
    next_index: (usize, usize),
}

impl<'a, T> Iterator for FrameIterator<'a, T>
where T: 'a {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<(usize, usize, &'a T)> {
        let (x, y) = self.next_index;

        if y < self.frame.width() {
            let val = self.frame.get(x, y);

            self.next_index =
                if x + 1 < self.frame.width() { (x + 1, y) }
                else { (0, y + 1) };

            Some((x, y, val))
        } else {
            None
        }
    }
}

impl<T> Frame<T> {
    /// Returns an iterator over tuples of coordinate and the element at that
    /// coordinate
    pub fn square_iterator(&self) -> FrameIterator<T> {
        FrameIterator {
            frame: &self,
            next_index: (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;

    #[test]
    fn frame_init() {
        let frame = Frame::<i32>::new(10, 10);
        assert_eq!(frame.width(), 10);
        assert_eq!(frame.height(), 10);
        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(*frame.get(x, y), i32::default());
            }
        }
    }

    #[test]
    fn frame_mut() {
        let mut frame = Frame::<i32>::new(2, 2);
        *frame.get_mut(1, 1) = 1;
        assert_eq!(*frame.get(1, 1), 1)
    }

    #[test]
    fn frame_next() {
        let mut frame1 = Frame::<i32>::new(2, 2);

        let frame2 = frame1.next_frame(|sq| { sq.get(0, 0) + 1 });

        let val = i32::default() + 1;
        *frame1.get_mut(0, 0) = val;
        *frame1.get_mut(0, 1) = val;
        *frame1.get_mut(1, 0) = val;
        *frame1.get_mut(1, 1) = val;

        assert_eq!(frame1, frame2);
    }
}
