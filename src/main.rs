use std::path::Path;

use clap::{Parser, Subcommand};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Round the corners of each tile in a spritesheet
    RoundCorners {
        /// Path to the input image
        input_image: String,

        /// Path to the output image
        output_image: Option<String>,

        /// Width of each tile in pixels
        #[arg(short = 'w', long = "width", default_value = "144")]
        width: u32,

        /// Height of each tile in pixels
        #[arg(short = 'h', long = "height", default_value = "190")]
        height: u32,

        /// Corner radius in pixels
        #[arg(short = 'r', long = "radius", default_value = "10")]
        radius: i32,
    },
    /// Resize an image to the desired scale
    Resize {
        /// The path to the input image
        input_image: String,

        /// The path to the resulting image
        output_image: String,

        /// The factor to scale the given spritesheet to
        #[arg(short = 's', long = "scale")]
        scale_factor: f32,
    },
    New {
        /// Width of each tile in pixels
        #[arg(short = 'w', long = "tile-width", default_value = "144")]
        tile_width: u32,

        /// Height of each tile in pixels
        #[arg(short = 'h', long = "tile-height", default_value = "190")]
        tile_height: u32,

        /// The total amount of columns in the new spritesheet
        #[arg(short = 'c', long = "columns")]
        columns: u32,

        /// The total amount of rows in the new spritesheet
        #[arg(short = 'r', long = "rows")]
        rows: u32,

        /// The path to the new spritesheet
        path: String,
    },
}

fn upscale_image(image: DynamicImage, scale_factor: f32) -> anyhow::Result<DynamicImage> {
    let (w, h) = image.dimensions();
    let nw = ((w as f32 * scale_factor) as u32).max(1);
    let nh = ((h as f32 * scale_factor) as u32).max(1);

    Ok(image.resize(nw, nh, image::imageops::FilterType::Lanczos3))
}

fn scale_image_file(in_path: &str, out_path: &str, scale_factor: f32) -> anyhow::Result<()> {
    let img = image::open(in_path)?;
    let scl = upscale_image(img, scale_factor)?;

    scl.save(out_path)?;

    Ok(())
}

fn mk_round_mask(width: u32, height: u32, radius: i32) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let mut mask = image::ImageBuffer::new(width, height);

    for pixel in mask.pixels_mut() {
        *pixel = image::Luma([255u8]);
    }

    let r = radius as u32;

    for y in 0..r {
        for x in 0..r {
            let dx = (r - 1 - x) as i32;
            let dy = (r - 1 - y) as i32;
            let distance_sq = dx * dx + dy * dy;
            let radius_sq = radius * radius;

            if distance_sq > radius_sq {
                mask.put_pixel(x, y, image::Luma([0u8]));
            }
        }
    }

    for y in 0..r {
        for x in (width - r)..width {
            let dx = (x - (width - r)) as i32;
            let dy = (r - 1 - y) as i32;
            let distance_sq = dx * dx + dy * dy;
            let radius_sq = radius * radius;

            if distance_sq > radius_sq {
                mask.put_pixel(x, y, image::Luma([0u8]));
            }
        }
    }

    for y in (height - r)..height {
        for x in 0..r {
            let dx = (r - 1 - x) as i32;
            let dy = (y - (height - r)) as i32;
            let distance_sq = dx * dx + dy * dy;
            let radius_sq = radius * radius;

            if distance_sq > radius_sq {
                mask.put_pixel(x, y, image::Luma([0u8]));
            }
        }
    }

    for y in (height - r)..height {
        for x in (width - r)..width {
            let dx = (x - (width - r)) as i32;
            let dy = (y - (height - r)) as i32;
            let distance_sq = dx * dx + dy * dy;
            let radius_sq = radius * radius;

            if distance_sq > radius_sq {
                mask.put_pixel(x, y, image::Luma([0u8]));
            }
        }
    }

    mask
}

fn round_tile_corners(tile: &RgbaImage, radius: i32) -> RgbaImage {
    let (width, height) = tile.dimensions();
    let mut rounded_tile = tile.clone();

    let mask = mk_round_mask(width, height, radius);

    // Apply the mask to the alpha channel
    for (x, y, pixel) in mask.enumerate_pixels() {
        let alpha_value = pixel[0];
        if let Some(rgba_pixel) = rounded_tile.get_pixel_mut_checked(x, y) {
            // Multiply the existing alpha by the mask value
            let current_alpha = rgba_pixel[3] as u16;
            let new_alpha = ((current_alpha * alpha_value as u16) / 255) as u8;
            rgba_pixel[3] = new_alpha;
        }
    }

    rounded_tile
}

fn process_image(
    in_path: &str,
    tile_width: u32,
    tile_height: u32,
    corner_radius: i32,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = image::open(in_path)?.to_rgba8();
    let (img_width, img_height) = img.dimensions();
    let tsx = img_width / tile_width;
    let tsy = img_height / tile_height;
    let mut output_img = RgbaImage::new(tsx * tile_width, tsy * tile_height);

    for y in 0..tsy {
        for x in 0..tsx {
            let left = x * tile_width;
            let top = y * tile_height;

            let tile =
                image::imageops::crop_imm(&img, left, top, tile_width, tile_height).to_image();
            let rounded_tile = round_tile_corners(&tile, corner_radius);

            image::imageops::overlay(&mut output_img, &rounded_tile, left as i64, top as i64);
        }
    }

    Ok(output_img)
}

fn new_sheet(
    tile_width: u32,
    tile_height: u32,
    columns: u32,
    rows: u32,
    output_path: &str,
) -> anyhow::Result<()> {
    let total_width = tile_width * columns;
    let total_height = tile_height * rows;

    let img: RgbaImage = ImageBuffer::from_fn(total_width, total_height, |_, _| Rgba([0, 0, 0, 0]));

    img.save(output_path)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let argv = Cli::parse();

    match argv.command {
        Subcommands::RoundCorners {
            input_image,
            output_image,
            width,
            height,
            radius,
        } => {
            if !Path::new(&input_image).exists() {
                eprintln!("[ERROR]: Input file '{}' does not exist", input_image);

                std::process::exit(1);
            }

            let path = Path::new(&input_image);
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let output_path = output_image.unwrap_or(format!("{}_rounded.png", stem));

            match process_image(&input_image, width, height, radius) {
                Ok(result) => {
                    result.save(&output_path)?;
                    println!("[SUCCESS]: Saved to '{}'", output_path);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("[ERROR]: Failed to process image: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Subcommands::Resize {
            input_image,
            output_image,
            scale_factor,
        } => scale_image_file(&input_image, &output_image, scale_factor),
        Subcommands::New {
            tile_height,
            tile_width,
            columns,
            rows,
            path: output_image,
        } => new_sheet(
            tile_width,
            tile_height,
            columns,
            rows,
            output_image.as_str(),
        ),
    }
}
