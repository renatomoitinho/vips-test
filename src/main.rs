use std::fs;
use std::time::Instant;
use libvips::ops;
use libvips::VipsApp;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let app = VipsApp::new("Custom-read", false).expect("Cannot initialize libvips");
    app.concurrency_set(4);
    app.cache_set_max(1024);
    app.cache_set_max_mem(1024);

    let path = "/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/500kb.jpg";
    
    let mut start = Instant::now();
    let buffer = fs::read(path).unwrap();
    //
    println!("time to read Local bytes time={:?} bytes={:?}", start.elapsed(), buffer.len() );

    // Load image
    start = Instant::now();
    //let image = VipsImage::image_new_from_buffer(&buffer, "").unwrap();
    let image = ops::thumbnail_buffer(&buffer, 512).unwrap();

    println!("time to read image from buffer w={:?} h={:?} time={:?} format={:?}", 
        image.get_width(), 
        image.get_height(), 
        start.elapsed(), 
        image.get_format());

    start = Instant::now();

    let options = ops::JpegsaveBufferOptions {
        q: 80,
        background: vec![255.0],
        strip: false,
        optimize_coding: false,
        optimize_scans: false,
        interlace: false,
        ..ops::JpegsaveBufferOptions::default()
    };

    let out_buffer = ops::jpegsave_buffer_with_opts(&image, &options).unwrap();

    println!("time to read buffer {:?} length={:?}", start.elapsed(), out_buffer.len() );

    // only write, no time

    let mut file = File::create("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/out_500kb.jpg")
    .expect("Error create file");
    file.write(&out_buffer).expect("no write");
    file.flush().expect("no flush");
    
}
