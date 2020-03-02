#[cfg(feature = "img")]
use image::DynamicImage;
use libwebp_sys::*;

use crate::shared::*;

/// An encoder for WebP images. It converts an uncompressed image into the WebP format.
/// The image data can be in any
/// It uses the default configuration of libwebp.
pub struct Encoder<'a> {
    image: &'a [u8],
    layout: PixelLayout,
    width: u32,
    height: u32,
}

impl<'a> Encoder<'a> {
    /// Creates a new encoder from the given image data.
    /// The image data must be in the pixel layout of the color parameter.
    pub fn new(image: &'a [u8], layout: PixelLayout, width: u32, height: u32) -> Self {
        Self { image, layout, width, height }
    }

    #[cfg(feature = "img")]
    /// Creates a new encoder from the given image.
    pub fn from_image(image: &'a DynamicImage) -> Self {
        match image {
            DynamicImage::ImageLuma8(_) => { unreachable!() }
            DynamicImage::ImageLumaA8(_) => { unreachable!() }
            DynamicImage::ImageRgb8(image) => {
                Self::from_rgb(image.as_ref(), image.width(), image.height())
            }
            DynamicImage::ImageRgba8(image) => {
                Self::from_rgba(image.as_ref(), image.width(), image.height())
            }
            DynamicImage::ImageBgr8(_) => { unreachable!() }
            DynamicImage::ImageBgra8(_) => { unreachable!() }
        }
    }

    /// Creates a new encoder from the given image data in the RGB pixel layout.
    pub fn from_rgb(image: &'a [u8], width: u32, height: u32) -> Self {
        Self { image, layout: PixelLayout::Rgb, width, height }
    }

    /// Creates a new encoder from the given image data in the RGBA pixel layout.
    pub fn from_rgba(image: &'a [u8], width: u32, height: u32) -> Self {
        Self { image, layout: PixelLayout::Rgba, width, height }
    }

    /// Encode the image with the given quality.
    /// The returned [WebPMemory](../shared/struct.WebPMemory.html) represents memory which is owned by libwebp
    /// and can be safely accessed through the Deref and DerefMut traits.
    /// The image quality must be between 0.0 and 100.0 inclusive for minimal and maximal quality respectively.
    pub fn encode(&self, quality: f32) -> Option<WebPMemory> {
        unsafe { encode(self.image, self.layout, self.width, self.height, quality) }
    }

    /// Encode the image losslessly.
    pub fn encode_lossless(&self) -> Option<WebPMemory> {
        unsafe { encode(self.image, self.layout, self.width, self.height, -1.0) }
    }
}

unsafe fn encode(image: &[u8], color: PixelLayout, width: u32, height: u32, quality: f32) -> Option<WebPMemory> {
    let width = width as _;
    let height = height as _;

    let mut buffer = std::ptr::null_mut::<u8>();

    let len = match color {
        PixelLayout::Rgb if quality < 0.0 => {
            let stride = width * 3;
            WebPEncodeLosslessRGB(image.as_ptr(), width, height, stride, &mut buffer as *mut _)
        }
        PixelLayout::Rgb => {
            let stride = width * 3;
            WebPEncodeRGB(image.as_ptr(), width, height, stride, quality, &mut buffer as *mut _)
        }
        PixelLayout::Rgba if quality < 0.0 => {
            let stride = width * 4;
            WebPEncodeLosslessRGBA(image.as_ptr(), width, height, stride, &mut buffer as *mut _)
        }
        PixelLayout::Rgba => {
            let stride = width * 4;
            WebPEncodeRGBA(image.as_ptr(), width, height, stride, quality, &mut buffer as *mut _)
        }
    };

    if len == 0 {
        None
    }else {
        Some(WebPMemory(buffer, len))
    }
}