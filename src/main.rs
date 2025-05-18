use anyhow::Result;
use mandelbrot::Mandelbrot;
use std::fs::File;

mod complex;
mod mandelbrot;

// プログラムを実行したときにここが最初に実行される関数となる(main 関数と呼ばれる)
#[tokio::main]
async fn main() -> Result<()> {
    let bounds = (15360, 8640);
    let upper_left = "-1.20+0.35i".parse()?;
    let lower_right = "-1+0.20i".parse()?;
    let file_name = "mandelbrot.png";

    let pixels = vec![0; bounds.0 * bounds.1];
    let mandelbrot = Mandelbrot::new(bounds, (upper_left, lower_right));
    let px = mandelbrot.render2(pixels).await?;

    let f = File::create(file_name)?;
    mandelbrot.write_image(f, &px)?;

    Ok(())
}
