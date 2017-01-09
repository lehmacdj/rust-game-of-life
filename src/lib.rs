extern crate rand;

/// Contains functions that deal with handling individual frames of a simulation
mod frame;

// Expose all of this at the root
pub use frame::*;

/// Public functions that define Conway's Game of Life
pub mod game_of_life;
pub mod two_color_life;
pub mod rainbow_life;
