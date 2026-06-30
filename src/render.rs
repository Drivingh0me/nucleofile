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

// Make this accept a closure from func_coninuous as arg
fn plot_func(res: Resolution) -> Result<RgbaImage> {
    let mut img = image::RgbaImage::new(res.w, res.h);
    Ok(img)
}

// save image with imgbuff.save("name.ext")?;

