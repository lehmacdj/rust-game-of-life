use super::Square;

use rand;
use rand::Rng;

use std::collections::HashMap;

/// Enum for a game of life that is multicolored with more complex rules
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Red,
    Green,
    Blue,
    Dead,
}

impl State {
    fn enemy(&self) -> Self {
        use self::State::{Red, Blue, Green, Dead};
        match *self {
            Red => Blue,
            Blue => Green,
            Green => Red,
            Dead => Dead,
        }
    }
}

impl<'a> Square<'a, State>
where State: 'a {
    /// Return the number of nodes alive surrounding this number
    fn alive_count(&self) -> usize {
        let this = self.get(0, 0);
//         println!("{:?}", self.within_ortholinear(1));
        self.within_ortholinear(1)
            .iter()
            .filter(|e| { **e != State::Dead && **e != this.enemy() })
            .count()
    }
}

impl Default for State {
    fn default() -> State { State::Dead }
}

pub fn rule(curr: Square<State>) -> State {
    use self::State::Dead;

    let curr_val = *curr.get(0, 0);

    if curr_val != Dead {
        match curr.alive_count() {
            2 | 3 => curr_val,
            _ => Dead,
        }
    } else {
        match curr.alive_count() {
            3 => {
                let neighbors = curr.within_ortholinear(1).iter().cloned()
                    .filter(|e| { *e != Dead })
                    .collect::<Vec<State>>();
                // println!("{:?}", neighbors);
                let m = mode(&neighbors[..]);

                // println!("{:?}", m);
                m
            },
            _ => Dead,
        }
    }
}

/// Return the most frequent thing in the slice. Break ties randomly or based on
/// some other metric
fn mode(data: &[State]) -> State {
    use std::cmp::Ordering::{Less, Equal, Greater};
    data.iter().cloned()
        // use a map to keep counts of each
        .fold(HashMap::new(),
              |mut map, e| { *map.entry(e).or_insert(0) += 1; map })
        .into_iter()
        // probably want to switch this back to keeping track of a Vec
        .fold(None, |acc, (s, n)| {
            acc.map_or(Some((n, s, 1)), |(n_, s_, c)| {
                match n.cmp(&n_) {
                    Greater => Some((n, s, 1)),
                    Equal   => Some((n_, s_, c + 1)),
                    Less    => Some((n_, s_, c)),
                }
            })
        })
        .map_or(random_color(), |(_, s, c)| {
            if c > 1 { random_color() } else { s }
        })
}

fn random_color() -> State {
    use self::State::{Red, Blue, Green};
    // Unwrap is safe since there will always be a result of some kind
    *rand::thread_rng().choose(&[Red, Blue, Green]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::super::Frame;
    use super::rule;
    use super::State;
    use super::State::{Red, Blue, Green, Dead};
    use super::mode;

    #[test]
    fn mode_test() {
        assert_eq!(mode(&[Red, Red, Red]), Red);
        assert_eq!(mode(&[Red, Red, Blue]), Red);
        assert_eq!(mode(&[Red, Red, Blue, Green]), Red);
        assert_eq!(mode(&[Blue, Green, Red, Green, Blue, Red, Blue]), Blue);
        assert!(mode(&[Red, Blue, Green]) != Dead);
    }

    #[test]
    fn oscillator_one_color() {
        // Create a board that should transform as shown:
        // DAD    DDD    DAD
        // DAD -> AAA -> DAD
        // DAD    DDD    DAD
        // the board is padded a lot because otherwise the overflow at the edges
        // would mess with the simulation
        let mut frame1 = Frame::<State>::new(4, 4);
        *frame1.get_mut(1, 0) = Red;
        *frame1.get_mut(1, 1) = Red;
        *frame1.get_mut(1, 2) = Red;

        let frame2 = frame1.next_frame(rule);
        let frame3 = frame2.next_frame(rule);

        let mut expected = Frame::<State>::new(4, 4);
        *expected.get_mut(0, 1) = Red;
        *expected.get_mut(1, 1) = Red;
        *expected.get_mut(2, 1) = Red;

        assert_eq!(frame2, expected);
        assert_eq!(frame3, frame1);
    }

    #[test]
    fn glider_one_color() {
        // Create a glider as pictured and follow its transformations:
        // DADDD
        // DDADD
        // AAADD
        // DDDDD
        // DDDDD
        let mut init = Frame::<State>::new(5, 5);
        *init.get_mut(1, 0) = Red;
        *init.get_mut(2, 1) = Red;
        *init.get_mut(0, 2) = Red;
        *init.get_mut(1, 2) = Red;
        *init.get_mut(2, 2) = Red;

        let mut second = Frame::<State>::new(5, 5);
        *second.get_mut(0, 1) = Red;
        *second.get_mut(2, 1) = Red;
        *second.get_mut(1, 2) = Red;
        *second.get_mut(2, 2) = Red;
        *second.get_mut(1, 3) = Red;

        let next = init.next_frame(rule);

        assert_eq!(next, second);
    }

    #[test]
    fn still_square_one_color() {
        // Make a square which shouldn't move in game of life
        let mut frame = Frame::<State>::new(3, 3);
        *frame.get_mut(0, 0) = Red;
        *frame.get_mut(0, 1) = Red;
        *frame.get_mut(1, 0) = Red;
        *frame.get_mut(1, 1) = Red;

        let frame_new = frame.next_frame(rule);

        assert_eq!(frame, frame_new);
    }
}
