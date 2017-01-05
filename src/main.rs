extern crate simulation;
extern crate image;

use std::fs::File;
use std::path::Path;

use simulation::game_of_life;
use simulation::game_of_life::State;

use image::Pixel;

type Color = image::Rgb<u8>;

fn main() {
    let side = 100;
    let img_side: u32 = (side * 10) as u32;
    let max_iters = 100;

    // create the frame
    let mut sim = simulation::Frame::<State>::new(side, side);
    sim.set(1, 0, State::Alive);
    sim.set(2, 1, State::Alive);
    sim.set(0, 2, State::Alive);
    sim.set(1, 2, State::Alive);
    sim.set(2, 2, State::Alive);

    for n in 0..max_iters {
        // write the image into a buffer
        let mut buf = image::ImageBuffer::new(img_side, img_side);
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let v = sim.get((x / 10) as usize, (y / 10) as usize);
            *pixel = W(v).into();
        }

        // save the image
        let num = format!("{:03}", n);
        let ref mut fout = File::create(&Path::new(&(num + ".png"))).unwrap();
        let _ = image::ImageRgb8(buf).save(fout, image::PNG);

        // advance to the next frame
        sim = sim.next_frame(game_of_life::rule);
    }
}

/// Wrapper to allow implementation of traits
struct W<T>(T);

impl Into<Color> for W<State> {
    fn into(self) -> Color {
        match self {
            W(State::Alive) => image::Luma([255]).to_rgb(),
            W(State::Dead) => image::Luma([0]).to_rgb(),
        }
    }
}
