use frame;

/// The state of a node in a GOL
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOLState {
    Alive,
    Dead,
}

impl Default for GOLState {
    fn default() -> GOLState { GOLState::Dead }
}

impl<'a> frame::Square<'a, GOLState>
where GOLState: 'a {
    /// Return the nodes adjacent to this square
    fn adjacent_nodes(&self) -> Vec<GOLState> {
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
            .filter(|e| { **e == GOLState::Alive })
            .count()
    }
}

/// The rule for Conway's Game of Life
pub fn game_of_life(square: frame::Square<GOLState>) -> GOLState {
    use self::GOLState::Alive;
    use self::GOLState::Dead;

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
