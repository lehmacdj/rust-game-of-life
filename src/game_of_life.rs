use super::Square;

/// The state of a node in a GOL
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

impl Default for State {
    fn default() -> State { State::Dead }
}

impl<'a> Square<'a, State>
where State: 'a {
    /// Return the number of nodes alive surrounding this number
    fn alive_count(&self) -> usize {
        self.within_ortholinear(1)
            .iter()
            .filter(|e| { **e == State::Alive })
            .count()
    }
}

/// The rule for Conway's Game of Life
pub fn rule(curr: Square<State>) -> State {
    use self::State::Alive;
    use self::State::Dead;

    if *curr.get(0, 0) == Alive {
        match curr.alive_count() {
            2 | 3 => Alive,
            _ => Dead,
        }
    } else {
        match curr.alive_count() {
            3 => Alive,
            _ => Dead,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Frame;
    use super::rule;
    use super::State;
    use super::State::Alive;

    #[test]
    fn oscillator() {
        // Create a board that should transform as shown:
        // DAD    DDD    DAD
        // DAD -> AAA -> DAD
        // DAD    DDD    DAD
        // the board is padded a lot because otherwise the overflow at the edges
        // would mess with the simulation
        let mut frame1 = Frame::<State>::new(4, 4);
        *frame1.get_mut(1, 0) = Alive;
        *frame1.get_mut(1, 1) = Alive;
        *frame1.get_mut(1, 2) = Alive;

        let frame2 = frame1.next_frame(rule);
        let frame3 = frame2.next_frame(rule);

        let mut expected = Frame::<State>::new(4, 4);
        *expected.get_mut(0, 1) = Alive;
        *expected.get_mut(1, 1) = Alive;
        *expected.get_mut(2, 1) = Alive;

        assert_eq!(frame2, expected);
        assert_eq!(frame3, frame1);
    }

    #[test]
    fn glider() {
        // Create a glider as pictured and follow its transformations:
        // DADDD
        // DDADD
        // AAADD
        // DDDDD
        // DDDDD
        let mut init = Frame::<State>::new(5, 5);
        *init.get_mut(1, 0) = Alive;
        *init.get_mut(2, 1) = Alive;
        *init.get_mut(0, 2) = Alive;
        *init.get_mut(1, 2) = Alive;
        *init.get_mut(2, 2) = Alive;

        let mut second = Frame::<State>::new(5, 5);
        *second.get_mut(0, 1) = Alive;
        *second.get_mut(2, 1) = Alive;
        *second.get_mut(1, 2) = Alive;
        *second.get_mut(2, 2) = Alive;
        *second.get_mut(1, 3) = Alive;

        let next = init.next_frame(rule);

        assert_eq!(next, second);
    }

    #[test]
    fn still_square() {
        // Make a square which shouldn't move in game of life
        let mut frame = Frame::<State>::new(3, 3);
        *frame.get_mut(0, 0) = Alive;
        *frame.get_mut(0, 1) = Alive;
        *frame.get_mut(1, 0) = Alive;
        *frame.get_mut(1, 1) = Alive;

        let frame_new = frame.next_frame(rule);

        assert_eq!(frame, frame_new);
    }
}
