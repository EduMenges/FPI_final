use image::{DynamicImage, GenericImageView, GenericImage};

fn camouflage_img(iB: &DynamicImage, iF: &DynamicImage, pos: (u32, u32)) -> DynamicImage {
    let camouflaged = DynamicImage::new_rgba8(iB.width(), iB.height());
    
    // Quantization and segmentation
    
    let luB = iB.to_luma8();
    let luF = iF.to_luma8();
    
    


    camouflaged
}

fn cut_image(basis: &DynamicImage, target: &DynamicImage, pos: (u32, u32)) -> DynamicImage {
    let mut cut = DynamicImage::new_rgba8(basis.width(), basis.height());

    for (x, y, pixel) in target.pixels() {
        if pixel.0[3] != 0 {
            cut.put_pixel(pos.0 + x, pos.1 + y, pixel);
        }
    }

    cut
}
