extern crate rand;
extern crate simulation;
extern crate image;

use std::fs::File;
use std::path::Path;

use std::ops::Deref;
use std::ops::DerefMut;

use simulation::game_of_life;
use simulation::game_of_life::State;

use rand::Rng;
use rand::Rand;

use image::Pixel;

type Color = image::Rgb<u8>;

fn main() {
    let side = 100;
    let img_side: u32 = (side * 10) as u32;
    let max_iters = 1000;

    // create the frame
    let mut sim = simulation::Frame::<State>::new(side, side);
    random_init_frame(&mut sim);

    // setup directory to contain images
    std::fs::create_dir_all("files").unwrap();

    for n in 0..max_iters {
        // write the image into a buffer
        let mut buf = image::ImageBuffer::new(img_side, img_side);
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let v = sim.get((x / 10) as usize, (y / 10) as usize);
            *pixel = W(*v).into();
        }

        // save the image
        let ref name = format!("files/{:03}.png", n);
        let ref mut fout = File::create(&Path::new(name)).unwrap();
        let _ = image::ImageRgb8(buf).save(fout, image::PNG);

        // advance to the next frame
        sim = sim.next_frame(game_of_life::rule);
    }
}

fn random_init_frame(frame: &mut simulation::Frame<State>) {
    for x in 0..frame.width() {
        for y in 0..frame.height() {
            frame.set(x, y, *rand::thread_rng().gen::<W<State>>());
        }
    }
}

/// Wrapper to allow implementation of traits
struct W<T>(T);

impl<T> Deref for W<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for W<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl Rand for W<State> {
    fn rand<R: Rng>(rng: &mut R) -> W<State> {
        match rng.gen::<bool>() {
            true => W(State::Alive),
            false => W(State::Dead),
        }
    }
}

impl Into<Color> for W<State> {
    fn into(self) -> Color {
        match self {
            W(State::Alive) => image::Luma([255]).to_rgb(),
            W(State::Dead) => image::Luma([0]).to_rgb(),
        }
    }
}
