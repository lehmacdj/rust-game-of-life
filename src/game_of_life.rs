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
    /// Return the nodes adjacent to this square
    fn adjacent_nodes(&self) -> Vec<State> {
        let mut nodes = vec![];
        for i in -1..2 {
            for j in -1..2 {
                if i != 0 || j != 0 {
                    nodes.push(*self.get(i, j));
                }
            }
        }
        nodes
    }

    /// Return the number of nodes alive surrounding this number
    fn alive_count(&self) -> usize {
        self.adjacent_nodes()
            .iter()
            .filter(|e| { **e == State::Alive })
            .count()
    }
}

/// The rule for Conway's Game of Life
pub fn rule(square: Square<State>) -> State {
    use self::State::Alive;
    use self::State::Dead;

    if *square.get(0, 0) == Alive {
        match square.alive_count() {
            2 | 3 => Alive,
            _ => Dead,
        }
    } else {
        match square.alive_count() {
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
        frame1.set(1, 0, Alive);
        frame1.set(1, 1, Alive);
        frame1.set(1, 2, Alive);

        let frame2 = frame1.next_frame(rule);
        let frame3 = frame2.next_frame(rule);

        let mut expected = Frame::<State>::new(4, 4);
        expected.set(0, 1, Alive);
        expected.set(1, 1, Alive);
        expected.set(2, 1, Alive);

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
        init.set(1, 0, Alive);
        init.set(2, 1, Alive);
        init.set(0, 2, Alive);
        init.set(1, 2, Alive);
        init.set(2, 2, Alive);

        let mut second = Frame::<State>::new(5, 5);
        second.set(0, 1, Alive);
        second.set(2, 1, Alive);
        second.set(1, 2, Alive);
        second.set(2, 2, Alive);
        second.set(1, 3, Alive);

        let next = init.next_frame(rule);

        assert_eq!(next, second);
    }

    #[test]
    fn still_square() {
        // Make a square which shouldn't move in game of life
        let mut frame = Frame::<State>::new(3, 3);
        frame.set(0, 0, Alive);
        frame.set(0, 1, Alive);
        frame.set(1, 0, Alive);
        frame.set(1, 1, Alive);

        let frame_new = frame.next_frame(rule);

        assert_eq!(frame, frame_new);
    }
}
