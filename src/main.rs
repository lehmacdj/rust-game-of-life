extern crate rand;
extern crate simulation;
extern crate image;

use std::fs::File;
use std::path::Path;

use std::ops::Deref;
use std::ops::DerefMut;

use simulation::rainbow_life;
use simulation::rainbow_life::State;

use rand::Rng;
use rand::Rand;

type Color = image::Rgb<u8>;

fn main() {
    let imgdim = 1000;
    let scale = 1;
    let side = (imgdim / scale) as usize;
    let max_iters = 1000;

    // create the frame
    let mut sim = simulation::Frame::new(side, side);
    random_init_frame(&mut sim);

    // setup directory to contain images
    std::fs::create_dir_all("files").unwrap();

    for n in 0..max_iters {
        // write the image into a buffer
        let mut buf = image::ImageBuffer::new(imgdim, imgdim);
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let v = sim.get((x / scale) as usize, (y / scale) as usize);
            *pixel = W(*v).into();
        }

        // save the image
        let ref name = format!("files/{:03}.png", n);
        let ref mut fout = File::create(&Path::new(name)).unwrap();
        let _ = image::ImageRgb8(buf).save(fout, image::PNG);

        // advance to the next frame
        sim = sim.next_frame(rainbow_life::rule);
    }
}

fn random_init_frame(frame: &mut simulation::Frame<State>) {
    for x in 0..frame.width() {
        for y in 0..frame.height() {
            *frame.get_mut(x, y)  = *rand::thread_rng().gen::<W<State>>();
        }
    }
}

/// Wrapper to allow implementation of traits for types defined outside of crate
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
        match rng.gen_range(0, 3) {
            0 => W(State::Red),
            1 => W(State::Blue),
            2 => W(State::Green),
            3 => W(State::Dead),
            _ => unreachable!(),
        }
    }
}

impl Into<Color> for W<State> {
    fn into(self) -> Color {
        match self {
            W(State::Red)   => image::Rgb([255, 0, 0]),
            W(State::Blue)  => image::Rgb([0, 255, 0]),
            W(State::Green) => image::Rgb([0, 0, 255]),
            W(State::Dead)  => image::Rgb([0, 0, 0]),
        }
    }
}
