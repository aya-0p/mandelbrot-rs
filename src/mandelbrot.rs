use anyhow::Result;
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use num::Complex;
use std::io::Write;

const LIMIT: u32 = 255;

pub struct Mandelbrot {
    bounds: (usize, usize),                   // (width, height)
    coordinate: (Complex<f64>, Complex<f64>), // (upper_left, lower_right) on complex plane
}

impl Mandelbrot {
    pub fn new(bounds: (usize, usize), coordinate: (Complex<f64>, Complex<f64>)) -> Self {
        Self { bounds, coordinate }
    }

    // TODO: implement
    // pub async fn parallel_render(&self, pixels: &mut [u8], thread_num: usize) {
    //     let rows_per_band = self.bounds.1 / thread_num + 1;
    //     let bands = pixels
    //         .chunks_mut(rows_per_band * self.bounds.0)
    //         .collect::<Vec<_>>();
    //     for (i, band) in bands.into_iter().enumerate() {
    //         let top = rows_per_band * i;
    //         let height = band.len() / self.bounds.0;
    //         let band_bounds = (self.bounds.0, height);
    //         let band_upper_left = pixel_to_point(self.bounds, self.coordinate, (0, top));
    //         let band_lower_right =
    //             pixel_to_point(self.bounds, self.coordinate, (self.bounds.0, top + height));
    //         tokio
    //     }
    // }

    // 集合の描画を行う
    pub fn render(&self, pixels: &mut [u8]) {
        for row in 0..self.bounds.1 {
            for col in 0..self.bounds.0 {
                let point = pixel_to_point(self.bounds, self.coordinate, (col, row));
                pixels[row * self.bounds.0 + col] = match calc_escape_time(point, LIMIT) {
                    Some(count) => (LIMIT - count) as u8,
                    None => 0,
                };
            }
        }
    }

    // png 画像として書き出しを行う。writer は Write trait を実装している型全てを許容する。
    pub fn write_image<W: Write>(&self, writer: W, pixels: &[u8]) -> Result<()> {
        let encoder = PngEncoder::new(writer);
        encoder.write_image(
            pixels,
            self.bounds.0 as u32,
            self.bounds.1 as u32,
            ColorType::L8,
        )?;
        Ok(())
    }
}

fn calc_escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    coordinate: (Complex<f64>, Complex<f64>),
    pixel: (usize, usize),
) -> Complex<f64> {
    let (width, height) = (
        coordinate.1.re - coordinate.0.re,
        coordinate.0.im - coordinate.1.im,
    );
    Complex::new(
        coordinate.0.re + pixel.0 as f64 * width / bounds.0 as f64,
        coordinate.0.im - pixel.1 as f64 * height / bounds.1 as f64,
    )
}
