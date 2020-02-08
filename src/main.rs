use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

use libvips::{ops, VipsImage};
use libvips::VipsApp;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::default::Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
        }
    }
}

fn get_position(width: i32, height: i32, max_size: i32) -> Position {
    let mut position = Position::default();

    if width < max_size {
        position.x = (max_size - width) / 2;
    } else if height < max_size {
        position.y = (max_size - height) / 2;
    }

    position
}

fn write_file_in_disk(buffer: &[u8], path: PathBuf) -> () {
    let mut _file = File::create(path).expect("Error create file");
    _file.write(buffer).expect("no write");
    _file.flush().expect("no flush");
}

fn get_jpeg_buffer(image: &VipsImage) -> Vec<u8> {
    let options = ops::JpegsaveBufferOptions {
        q: 90,
        background: vec![255.0],
        strip: true,
        optimize_coding: true,
        optimize_scans: true,
        interlace: true,
        ..ops::JpegsaveBufferOptions::default()
    };

    ops::jpegsave_buffer_with_opts(image, &options).unwrap()
}

fn extend(image: &VipsImage, position: &Position, square: i32) -> VipsImage {
    let ops = ops::EmbedOptions {
        extend: ops::Extend::White,
        ..ops::EmbedOptions::default()
    };

    ops::embed_with_opts(image, position.x, position.y, square, square, &ops)
        .unwrap()
}

fn thumb(image: &VipsImage, with: i32) -> VipsImage {
    ops::thumbnail_image(image, with).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cpus = num_cpus::get();
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    app.concurrency_set(cpus as i32);
    app.cache_set_max(0);
    app.cache_set_max_mem(0);


    let path: &Path = Path::new(args[1].as_str());
    let square: i32 = args[2].parse().unwrap();
    let buffer = fs::read(path).expect("Not file Found.");

    // Load image
    let total_start = Instant::now();
    let mut start = Instant::now();
    let mut image = VipsImage::image_new_from_buffer(&buffer[..], "[access=VIPS_ACCESS_SEQUENTIAL]").unwrap();
    println!("time to read image from buffer w={:?} h={:?} time={:?} format={:?}",
             image.get_width(),
             image.get_height(),
             start.elapsed(),
             image.get_format()
    );

    // Resize
    start = Instant::now();
    image = thumb(&image, square);
    println!("time to resize time={:?} width={:?} heigth={:?}",
             start.elapsed(),
             image.get_width(),
             image.get_height()
    );

    // Extend
    start = Instant::now();
    let position = get_position(image.get_width(), image.get_height(), square);
    image = extend(&image, &position, square);
    println!("time to extend time={:?} width={:?} heigth={:?}",
             start.elapsed(),
             image.get_width(),
             image.get_height()
    );

    // Write buffer [slow :(]
    start = Instant::now();
    let out_buffer = get_jpeg_buffer(&image);
    println!("time to read buffer {:?} length={:?}",
             start.elapsed(),
             out_buffer.len());

    println!("total time {:?}", total_start.elapsed());

    // Only save [not important]
    write_file_in_disk(&out_buffer, path.parent().unwrap()
        .join(format!("out_{}__.jpeg", path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        )));
}
