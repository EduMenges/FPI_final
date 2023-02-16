use image::{DynamicImage, GenericImage, GenericImageView};

use crate::{
    quantization::quantize_by_tones,
    segmentation::{ImageSegments, ImgSegmentation},
};

fn camouflage_img(i_b: &DynamicImage, i_f: &DynamicImage, pos: (u32, u32)) -> DynamicImage {
    let camouflaged = DynamicImage::new_rgba8(i_b.width(), i_b.height());

    // Quantization and segmentation

    // Converting to luminance
    let mut lu_b = i_b.to_luma_alpha8();
    let mut lu_f = i_f.to_luma_alpha8();

    // Applying quantization
    quantize_by_tones(&mut lu_b);
    quantize_by_tones(&mut lu_f);

    // Cropping the background
    let mut lu_b = crop_image(
        &DynamicImage::ImageLumaA8(lu_b),
        &DynamicImage::ImageLumaA8(lu_f.clone()),
        pos,
    )
    .to_luma_alpha8();

    // Segmenting images
    let seg_b = ImgSegmentation::segment_img(&lu_b);
    let seg_f = ImgSegmentation::segment_img(&lu_f);

    // Creating graphs

    camouflaged
}

/// Crops the basis image with the shape of the target image.
fn crop_image(basis: &DynamicImage, target: &DynamicImage, pos: (u32, u32)) -> DynamicImage {
    let mut cut = DynamicImage::new_rgba8(basis.width(), basis.height());

    for (x, y, pixel) in target.pixels() {
        if pixel.0[3] > 0 {
            cut.put_pixel(pos.0 + x, pos.1 + y, pixel);
        }
    }

    cut
}
