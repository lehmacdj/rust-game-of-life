#[cfg(test)]
mod tests;

/// Contains functions that deal with handling individual frames of a simulation
pub mod frame;
/// Contains functions that represent rulesets for simulations.
/// These generally consist of one data type and one function that transforms
/// any given square to a new square based on the surrounding region of tiles.
pub mod rules;
