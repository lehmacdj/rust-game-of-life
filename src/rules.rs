use frame;

/// The state of a node in a GOL
#[derive(Clone, Debug, PartialEq)]
pub enum GOLState {
    Alive,
    Dead,
}

impl Square<GOLState> {
    /// Return the nodes adjacent to this square
    fn adjacent_nodes(&self) -> Vec<GOLState> {
        let mut nodes = vec![];
        nodes.push(self.get(0, -1));
        nodes.push(self.get(0, 1));
        nodes.push(self.get(-1, 0));
        nodes.push(self.get(1, 0));
        nodes
    }

    /// Return the number of nodes alive surrounding this number
    fn alive_count(&self) -> usize {
        self.adjacent_nodes()
            .iter()
            .filter(|e| { e == Alive })
            .count()
    }
}

/// The rule for Conway's Game of Life
pub fn game_of_life(square: Square<GOLState>) {
    if square.get(0, 0) == Alive {
        match square.alive_count() {
            0 | 1 | 4 => Dead,
            _ => Alive,
        }
    } else {
        match square.alive_count() {
            3 => Alive,
            _ => Dead,
        }
    }
}
