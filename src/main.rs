#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

//! Raytracer in a weekend attempt?

type Pixel = (f64, f64, f64);
type Image = Vec<Vec<Pixel>>;

fn main() {
    let image: Image = make_test_image();
    let output_ppm = to_ppm(image);
    println!("{}", output_ppm);
}

fn make_test_image() -> Image {
    let (width, height) = (256, 256);

    let mut image = vec![];

    for row in 0..height {
        let mut image_row = vec![];
        for col in 0..width {
            let pixel = (row as f64/256.0, col as f64/256.0, 0.0);
            image_row.push(pixel);
        }
        image.push(image_row);
    }

    return image;
}

fn to_ppm(image: Image) -> String {
    let (width, height) = (image[0].len(), image.len());

    let mut ppm_string: String = "P3\n".to_string();
    ppm_string.push_str(format!("{} {}\n", width, height).as_str());
    ppm_string.push_str("255\n");

    for row in 0..height {
        for col in 0..width {
            let (r, g, b) = image[row][col];
            let (r_int, g_int, b_int) = (
                (r * 255.0).round(),
                (g * 255.0).round(),
                (b * 255.0).round(),
            );
            ppm_string.push_str(format!("{} {} {}", r_int, g_int, b_int).as_str());
            ppm_string.push_str(" ");
        }
        ppm_string.push_str("\n");
    }

    return ppm_string;
}
