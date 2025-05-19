use anyhow::Result;
use mandelbrot::Mandelbrot;
use std::fs::File;

mod complex;
mod mandelbrot;

// プログラムを実行したときにここが最初に実行される関数となる(main 関数と呼ばれる)
#[tokio::main]
async fn main() -> Result<()> {
    let size = 240 * 32;
    let bounds = (3 * size, 2 * size);
    let upper_left = "-2+1i".parse()?;
    let lower_right = "1-1i".parse()?;
    let file_name = "mandelbrot.png";

    let pixels = vec![0; bounds.0 * bounds.1];
    let mandelbrot = Mandelbrot::new(bounds, (upper_left, lower_right));
    let px = mandelbrot.render2(pixels).await?;

    let f = File::create(file_name)?;
    mandelbrot.write_image(f, &px)?;

    Ok(())
}
