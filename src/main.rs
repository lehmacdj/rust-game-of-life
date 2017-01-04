extern crate simulation;
extern crate image;

use std::fs::File;
use std::path::Path;

use simulation::frame;
use simulation::rules;
use simulation::rules::GOLState;
use simulation::rules::GOLState::Alive;
use simulation::rules::GOLState::Dead;

type Color = image::Luma<u8>;

fn main() {
    let side = 100;
    let img_side: u32 = (side * 10) as u32;
    let max_iters = 100;

    // create the frame
    let mut frame = frame::Frame::<GOLState>::new(side, side);
    frame.set(1, 0, Alive);
    frame.set(2, 1, Alive);
    frame.set(0, 2, Alive);
    frame.set(1, 2, Alive);
    frame.set(2, 2, Alive);

    for n in 0..max_iters {
        // write the image into a buffer
        let mut imgbuf = image::ImageBuffer::new(img_side, img_side);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let v = frame.get((x / 10) as usize, (y / 10) as usize);
            *pixel = W(v).into();
        }

        // save the image
        let ref mut fout = File::create(&Path::new(&(n.to_string() + ".png"))).unwrap();
        let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);

        // advance to the next frame
        frame = frame.next_frame(rules::game_of_life);
    }
}

/// Wrapper to allow implementation of traits
struct W<T>(T);

impl Into<Color> for W<GOLState> {
    fn into(self) -> Color {
        match self {
            W(Alive) => image::Luma([std::u8::MAX]),
            W(Dead) => image::Luma([0]),
        }
    }
}
