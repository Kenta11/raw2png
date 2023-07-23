use image::{ImageBuffer, Rgb, RgbImage};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // read a path
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <FILE>", args[0]);
        process::exit(1);
    };

    // read a raw image
    let mut f = File::open(path)?;
    let mut v = vec![];
    f.read_to_end(&mut v)?;

    // save the image as png
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = Rgb([
                v[((y * WIDTH + x) * 3) as usize],
                v[((y * WIDTH + x) * 3 + 1) as usize],
                v[((y * WIDTH + x) * 3 + 2) as usize],
            ]);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("./imagedata.png")?;

    Ok(())
}
