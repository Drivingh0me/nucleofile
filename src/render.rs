// This is where the image drawing stuff happens.
// Also image exporting happens here.
use crate::compute;
use crate::error;
use image::RgbaImage;

use error::Result;

#[derive(Debug)]
pub struct Resolution {
    pub w: u32,
    pub h: u32,
}

// Accepts closure from compute::Func_continuous
fn plot_func<F: Fn()>(res: Resolution, f: F) -> Result<RgbaImage> {
    let mut img = image::RgbaImage::new(res.w, res.h);
    Ok(img)
}

// save image with imgbuff.save("name.ext")?;

