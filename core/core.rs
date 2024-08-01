// █ █▄ ▄█ █▀▄ ▄▀▄ █▀▄ ▀█▀ ▄▀▀
// █ █ ▀ █ █▀  ▀▄▀ █▀▄  █  ▄██
use image::imageops::contrast;
use image::{DynamicImage, ImageBuffer};
use rand::Rng;
use std::io::Cursor;
use std::panic;
use wasm_bindgen::prelude::*;

// █▀▄ ██▀ ▄▀▄ █▀▄   █ █▄ ▄█ ▄▀▄ ▄▀  ██▀   █▀ █▀▄ ▄▀▄ █▄ ▄█   ██▄ ▀▄▀ ▀█▀ ██▀ ▄▀▀
// █▀▄ █▄▄ █▀█ █▄▀   █ █ ▀ █ █▀█ ▀▄█ █▄▄   █▀ █▀▄ ▀▄▀ █ ▀ █   █▄█  █   █  █▄▄ ▄██
fn read_image(image_data: &[u8]) -> Result<DynamicImage, image::ImageError> {
    let img = image::load_from_memory(image_data);
    img.map(|img| DynamicImage::ImageRgba8(contrast(&img, 5.0)))
}

// ▄▀▀ ▄▀▄ █ █ ██▀   █ █▄ ▄█ ▄▀▄ ▄▀  ██▀
// ▄██ █▀█ ▀▄▀ █▄▄   █ █ ▀ █ █▀█ ▀▄█ █▄▄
fn save_img(pixels: ImageBuffer<image::Rgb<u8>, Vec<u8>>, pixels_vec: Vec<u8>) -> Vec<u8> {
    let new_img: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(pixels.width(), pixels.height(), pixels_vec).unwrap();
    let output_image = DynamicImage::from(new_img);
    let output_image = DynamicImage::ImageRgba8(contrast(&output_image, 20.0));
    let mut output_bytes: Vec<u8> = Vec::new();
    output_image
        .write_to(
            &mut Cursor::new(&mut output_bytes),
            image::ImageOutputFormat::Png,
        )
        .unwrap();

    output_bytes
}

// ██▀ █▀▄ ▄▀  ██▀   █▀▄ ██▀ ▀█▀ ██▀ ▄▀▀ ▀█▀ █ ▄▀▄ █▄ █
// █▄▄ █▄▀ ▀▄█ █▄▄   █▄▀ █▄▄  █  █▄▄ ▀▄▄  █  █ ▀▄▀ █ ▀█
fn detect_edges(img: &DynamicImage) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let detection = edge_detection::canny(
        img.to_luma8(),
        1.2,  // sigma
        0.2,  // strong threshold
        0.01, // weak threshold
    );
    detection.as_image().clone().into_rgb8()
}

// ▄▀▀ ▄▀▄ █   ▄▀▀ █ █ █   ▀█▀ ▄▀▄ ▀█▀ ██▀   ▄▀▄ █ █ ██▀ █▀▄ ▄▀▄ ▄▀  ██▀   ▄▀▀ ▄▀▄ █   ▄▀▄ █ █ █▀▄
// ▀▄▄ █▀█ █▄▄ ▀▄▄ ▀▄█ █▄▄  █  █▀█  █  █▄▄   █▀█ ▀▄▀ █▄▄ █▀▄ █▀█ ▀▄█ █▄▄   ▀▄▄ ▀▄▀ █▄▄ ▀▄▀ ▀▄█ █▀▄
fn calculate_avg(
    total_red: u32,
    total_green: u32,
    total_blue: u32,
    pixels: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> ([u8; 3], [u8; 3]) {
    let avg_red = total_red / (pixels.width() * pixels.height());
    let avg_green = total_green / (pixels.width() * pixels.height());
    let avg_blue = total_blue / (pixels.width() * pixels.height());
    let mut averages = [(avg_red, "r"), (avg_green, "g"), (avg_blue, "b")];
    averages.sort();

    let mut saturate = [0, 0, 0];

    if averages[0].1 == "r" {
        saturate = [255, 0, 0];
    } else if averages[0].1 == "g" {
        saturate = [0, 255, 0];
    } else if averages[0].1 == "b" {
        saturate = [0, 0, 255];
    }

    let inverted_average: [u8; 3] = [
        (255 - saturate[0]),
        (255 - saturate[1]),
        (255 - saturate[2]),
    ];

    (saturate, inverted_average)
}

// █▀ █ █▀▄ ▄▀▀ ▀█▀   █▀▄ ▄▀▄ ▄▀▀ ▄▀▀   ▄▀▄ █▀   █▀▄ █▀▄ ▄▀▄ ▄▀▀ ██▀ ▄▀▀ ▄▀▀ █ █▄ █ ▄▀
// █▀ █ █▀▄ ▄██  █    █▀  █▀█ ▄██ ▄██   ▀▄▀ █▀   █▀  █▀▄ ▀▄▀ ▀▄▄ █▄▄ ▄██ ▄██ █ █ ▀█ ▀▄█
fn pre_process(mut pixels_vec: Vec<u8>) -> (Vec<u8>, (u32, u32, u32)) {
    let (mut total_red, mut total_green, mut total_blue) = (0, 0, 0);
    for pixel in pixels_vec.chunks_exact_mut(3) {
        pixel[0] /= 6;
        pixel[1] /= 6;
        pixel[2] /= 6;

        let mut rgb = [(pixel[0], "r"), (pixel[1], "g"), (pixel[2], "b")];
        rgb.sort_by(|a, b| a.0.cmp(&b.0));
        if rgb[2].0 > rgb[1].0 {
            rgb[2].0 *= 6;
        } else if rgb[1].0 > rgb[0].0 {
            rgb[2].0 *= 6;
            rgb[1].0 *= 6;
        }
        // set pixel[0] to r and pixel[1] to g and pixel[2] to b
        if rgb[0].1 == "r" {
            pixel[0] = rgb[0].0;
        } else if rgb[0].1 == "g" {
            pixel[1] = rgb[0].0;
        } else if rgb[0].1 == "b" {
            pixel[2] = rgb[0].0;
        }
        if rgb[1].1 == "r" {
            pixel[0] = rgb[1].0;
        } else if rgb[1].1 == "g" {
            pixel[1] = rgb[1].0;
        } else if rgb[1].1 == "b" {
            pixel[2] = rgb[1].0;
        }
        if rgb[2].1 == "r" {
            pixel[0] = rgb[2].0;
        } else if rgb[2].1 == "g" {
            pixel[1] = rgb[2].0;
        } else if rgb[2].1 == "b" {
            pixel[2] = rgb[2].0;
        }
        let mut rng = rand::thread_rng();
        if (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) > 40
            && (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) < 200
        {
            if rng.gen_bool(0.2) {
                pixel[0] = rng.gen_range(0..100);
                pixel[1] = rng.gen_range(0..100);
                pixel[2] = rng.gen_range(0..100);
            }
        } else if rng.gen_bool(0.05) {
            pixel[0] = rng.gen_range(0..255);
            pixel[1] = rng.gen_range(0..255);
            pixel[2] = rng.gen_range(0..255);
        }
        total_red += pixel[0] as u32;
        total_green += pixel[1] as u32;
        total_blue += pixel[2] as u32;
    }
    (pixels_vec, (total_red, total_green, total_blue))
}

// ▄▀▀ ██▀ ▄▀▀ ▄▀▄ █▄ █ █▀▄   █▀▄ ▄▀▄ ▄▀▀ ▄▀▀   ▄▀▄ █▀   █▀▄ █▀▄ ▄▀▄ ▄▀▀ ██▀ ▄▀▀ ▄▀▀ █ █▄ █ ▄▀
// ▄██ █▄▄ ▀▄▄ ▀▄▀ █ ▀█ █▄▀   █▀  █▀█ ▄██ ▄██   ▀▄▀ █▀   █▀  █▀▄ ▀▄▀ ▀▄▄ █▄▄ ▄██ ▄██ █ █ ▀█ ▀▄█
fn post_process(
    mut pixels_vec: Vec<u8>,
    edge_detection_pixels_vec: Vec<u8>,
    saturate: [u8; 3],
    inverted_average: [u8; 3],
) -> Vec<u8> {
    let mut i = 0;
    for pixel in pixels_vec.chunks_exact_mut(3) {
        let mut rgb = [pixel[0], pixel[1], pixel[2]];
        rgb.sort();
        if rgb[2] <= rgb[1]
            && rgb[1] <= rgb[0]
            && (rgb[0] + 10) as u16 + (rgb[1] + 10) as u16 + (rgb[2] + 10) as u16 > 10
        {
            pixel[0] = saturate[0];
            pixel[1] = saturate[1];
            pixel[2] = saturate[2];
        }

        if edge_detection_pixels_vec[i] as u16
            + edge_detection_pixels_vec[i + 1] as u16
            + edge_detection_pixels_vec[i + 2] as u16
            != 0
        {
            pixel[0] = inverted_average[0];
            pixel[1] = inverted_average[1];
            pixel[2] = inverted_average[2];
        }
        i += 3;

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        if pixel[0] < 85 {
            pixel[0] = 0;
        } else if pixel[0] < 170 {
            if rng.gen_bool(0.5) {
                pixel[0] = 0;
            } else {
                pixel[0] = 255;
            }
        } else {
            pixel[0] = 255;
        }

        if pixel[1] < 85 {
            pixel[1] = 0;
        } else if pixel[1] < 170 {
            if rng.gen_bool(0.5) {
                pixel[1] = 0;
            } else {
                pixel[1] = 255;
            }
        } else {
            pixel[1] = 255;
        }

        if pixel[2] < 85 {
            pixel[2] = 0;
        } else if pixel[2] < 170 {
            if rng.gen_bool(0.5) {
                pixel[2] = 0;
            } else {
                pixel[2] = 255;
            }
        } else {
            pixel[2] = 255;
        }
    }

    pixels_vec
}

// █ █▄ █ █ █ ██▀ █▀▄ ▀█▀   █▀▄ ▄▀▄ █▄ █ █▀▄ ▄▀▄ █▄ ▄█   ▄▀▀ █▄█ █ █ █▄ █ █▄▀ ▄▀▀
// █ █ ▀█ ▀▄▀ █▄▄ █▀▄  █    █▀▄ █▀█ █ ▀█ █▄▀ ▀▄▀ █ ▀ █   ▀▄▄ █ █ ▀▄█ █ ▀█ █ █ ▄██
fn invert_chunks(mut pixels_vec: Vec<u8>, pixels_width: u32, pixels_height: u32) -> Vec<u8> {
    // invert random 16x16 chunks
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let x = (rng.gen::<f32>() * (pixels_width - 16 * 3) as f32).round() as u32 / 16 * 16;
        let y = (rng.gen::<f32>() * (pixels_height - 16 * 3) as f32).round() as u32 / 16 * 16;
        for i in 0..16 {
            for j in 0..16 {
                let pixel_index = ((x + i) * 3 + (y + j) * pixels_width * 3) as usize;
                pixels_vec[pixel_index] = 255 - pixels_vec[pixel_index];
                pixels_vec[pixel_index + 1] = 255 - pixels_vec[pixel_index + 1];
                pixels_vec[pixel_index + 2] = 255 - pixels_vec[pixel_index + 2];
            }
        }
    }
    pixels_vec
}

// █▀▄ █▀▄ ▄▀▄ ▄▀▀ ██▀ ▄▀▀ ▄▀▀   █ █▄ ▄█ ▄▀▄ ▄▀  ██▀
// █▀  █▀▄ ▀▄▀ ▀▄▄ █▄▄ ▄██ ▄██   █ █ ▀ █ █▀█ ▀▄█ █▄▄
#[wasm_bindgen]
pub fn process(input_image: Vec<u8>) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut image_bytes: Vec<u8> = include_bytes!("tmp3.jpg").to_vec();
    if input_image.len() > 0 {
        image_bytes = input_image;
    }
    
    let img = match read_image(&image_bytes) {
        Ok(img) => img,
        Err(err) => {
            println!("{}", err);
            return vec![];
        }
    };

    let edge_detection = detect_edges(&img);
    let pixels = img.into_rgb8();
    let edge_detection_pixels = &edge_detection;
    let pixels_vec = pixels.clone().into_vec();
    let edge_detection_pixels_vec = edge_detection_pixels.clone().into_vec();

    let (pixels_vec, (total_red, total_green, total_blue)) = pre_process(pixels_vec.clone());

    let avg_red = total_red / (pixels.width() * pixels.height()) as u32;
    let avg_green = total_green / (pixels.width() * pixels.height()) as u32;
    let avg_blue = total_blue / (pixels.width() * pixels.height()) as u32;
    let mut averages = [(avg_red, "r"), (avg_green, "g"), (avg_blue, "b")];
    averages.sort();

    let (saturate, inverted_average) =
        calculate_avg(total_red, total_green, total_blue, pixels.clone());

    let pixels_vec = post_process(
        pixels_vec.clone(),
        edge_detection_pixels_vec,
        saturate,
        inverted_average,
    );

    let pixels_vec = invert_chunks(pixels_vec.clone(), pixels.width(), pixels.height());

    save_img(pixels, pixels_vec)
}
