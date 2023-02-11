use image::{LumaA, GrayAlphaImage};

pub fn quantize(img: &mut GrayAlphaImage, n: u8) {
    let t_1 = img.pixels().min_by_key(|p| p[0]).unwrap().0[0];
    let t_2 = img.pixels().max_by_key(|p| p[0]).unwrap().0[0];

    let tam_int = t_2 - t_1;

    if n < tam_int {
        let tb: f64 = tam_int as f64 / n as f64;
        let center_offset: f64 = tam_int as f64 / (2 * n) as f64;
        let mut bin_vec: Vec<u8> = Vec::with_capacity(n as usize + 1);

        for i in (t_1..t_2).step_by(tb.round() as usize) {
            bin_vec.push(i + center_offset.round() as u8);
        }
        bin_vec.push(*bin_vec.last().unwrap());

        for h in 0..img.height() {
            for w in 0..img.width() {
                let pixel = img.get_pixel(w, h);
                let bin_i = usize::from(pixel[0] - t_1) / tb.round() as usize;
                let quantized_value = bin_vec[bin_i];

                let quantized_pixel =
                    LumaA::<u8>([quantized_value, pixel[1]]);

                img.put_pixel(w, h, quantized_pixel);
            }
        }
    }
}
