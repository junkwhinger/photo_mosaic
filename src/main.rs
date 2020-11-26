extern crate image;

use image::{ImageBuffer, GenericImageView, imageops};

fn main() {
    println!("Photo Mosaic");

    println!("[Step1] prepare the source image.");
    // 1) open
    let image_path = "data/source/src.jpg";
    let src_img  = match image::open(image_path) {
        Ok(v) => v,
        Err(e) => panic!("failed to load image: {}", e),
    };
    let (src_img_height, src_img_width ) = src_img.dimensions();
    println!("source dimensions width: {} height: {}", src_img_height, src_img_width);

    let enlarge_ratio = 5;

    // 2) enlarge it to the desired size
    // imageops::resize(img, )
    let resized_src_img = src_img.resize(src_img_width * enlarge_ratio, 
        src_img_height * enlarge_ratio,
        imageops::FilterType::Nearest);

    let (resized_img_height, resized_img_width ) = resized_src_img.dimensions();

    println!("resized dimensions width: {} height: {}", resized_img_height, resized_img_width);
    let _ = resized_src_img.save("data/test.jpg");

}
