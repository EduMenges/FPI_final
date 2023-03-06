use image::{DynamicImage, GenericImageView};

use crate::{
    quantization::quantize_by_tones,
    segmentation::{ImgSegmentation}, helpers::Crop, graphs::mount_graph,
};

fn camouflage_img(i_b: &DynamicImage, i_f: &DynamicImage, _pos: (u32, u32)) -> DynamicImage {
    let camouflaged = DynamicImage::new_rgba8(i_b.width(), i_b.height());

    // Quantization and segmentation

    // Converting to luminance
    let mut lu_b = i_b.to_luma_alpha8();
    let mut lu_f = i_f.to_luma_alpha8();

    // Applying quantization
    quantize_by_tones(&mut lu_b);
    quantize_by_tones(&mut lu_f);

    // Segmenting images
    let seg_b = ImgSegmentation::segment_img(&lu_b);
    let seg_f = ImgSegmentation::segment_img(&lu_f);

    // Cropping images
    let seg_b = seg_f.crop(seg_b);
    
    // Creating graphs
    let _graph = mount_graph(seg_f, seg_b);
    
    camouflaged
}
