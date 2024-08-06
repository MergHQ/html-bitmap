use core::f32;
use image::{GenericImageView, ImageReader, Pixel, RgbaImage};
use std::{fs::File, io::Write};

fn image_buffer_to_html(image_buffer: &RgbaImage, dimensions: (u32, u32)) -> String {
    let mut out = String::new();
    let (pixels_x, pixels_y) = dimensions;

    for y in 0..pixels_y {
        out.push_str("<div style=\"display: flex; flex-direction: row;\">");

        for x in 0..pixels_x {
            let [r, g, b, a] = image_buffer.get_pixel(x, y).channels() else {
                panic!("Unknown pixel channels")
            };

            let element = format!(
                "<div style=\"background-color: rgba({r}, {g}, {b}, {a}); width: 1px; height: 1px;\"></div>",
                r = r,
                g = g,
                b = b,
                a = *a as f32 / 255.0f32
            );

            out.push_str(&element);
        }

        out.push_str("</div>");
    }

    out
}

fn main() {
    let source_file = std::env::args().nth(1).expect("no source file provided");

    let img = ImageReader::open(source_file)
        .expect("Could not open image")
        .decode()
        .expect("Could not decode image");
    let img_dimensions = img.dimensions();

    let buffer = img.as_rgba8().expect("Could not read image file");

    let out_str = image_buffer_to_html(buffer, img_dimensions);

    let mut out_file = File::create("out.html").expect("Could not create out file");

    out_file
        .write_all(out_str.as_bytes())
        .expect("Could not write output image")
}
