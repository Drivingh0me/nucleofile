// This is where the image drawing stuff happens.
// Also image exporting happens here.
use crate::math;
use image::RgbaImage;

pub struct Resolution {
    w: u32,
    h: u32,
}

fn plot_func(f: math::Func, res: Resolution) -> Result<RgbaImage> {
    let mut img = image::RgbaImage::new(res.w, res.h);
    img
}

// save image with imgbuff.save("name.ext")?;

