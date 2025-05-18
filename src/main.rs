use anyhow::Result;
use gif::{Encoder, Frame, Repeat};
use image::{DynamicImage, RgbaImage};
use std::fs::File;

fn main() -> Result<()> {
    let img1 = image::open("assets/images/1.jpeg")?
        .resize_exact(640, 360, image::imageops::FilterType::Lanczos3);
    let img2 = image::open("assets/images/2.jpeg")?
        .resize_exact(640, 360, image::imageops::FilterType::Lanczos3);

    create_gif_in_memory(&img1, &img2, "out.gif")?;

    Ok(())
}

fn blend_images(img1: &RgbaImage, img2: &RgbaImage, t: f32) -> RgbaImage {
    let (w, h) = img1.dimensions();
    let mut blended = RgbaImage::new(w, h);

    let buf1 = img1.as_raw();
    let buf2 = img2.as_raw();
    let buf_blend = blended.as_mut();

    for i in 0..(w * h) as usize {
        let idx = i * 4;
        for c in 0..3 {
            buf_blend[idx + c] =
                ((1.0 - t) * buf1[idx + c] as f32 + t * buf2[idx + c] as f32) as u8;
        }
        buf_blend[idx + 3] = 255;
    }

    blended
}

fn create_gif_in_memory(img1: &DynamicImage, img2: &DynamicImage, output_path: &str) -> Result<()> {
    let img1 = img1.to_rgba8();
    let img2 = img2.to_rgba8();

    let mut image = File::create(output_path)?;
    let mut encoder = Encoder::new(&mut image, 640, 360, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    let steps = 30;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let blended = blend_images(&img1, &img2, t);

        let mut frame = Frame::from_rgba_speed(640, 360, &mut blended.clone().into_raw(), 10);
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
