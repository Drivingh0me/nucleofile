// This is where the image drawing stuff happens.
// Also image exporting happens here.
use crate::math;
use crate::error;
use image::RgbaImage;

use error::Result;

#[derive(Debug)]
pub struct Resolution {
    pub w: u32,
    pub h: u32,
}

fn plot_func(f: math::Func, res: Resolution) -> Result<RgbaImage> {
    let mut img = image::RgbaImage::new(res.w, res.h);
    Ok(img)
}

// save image with imgbuff.save("name.ext")?;

