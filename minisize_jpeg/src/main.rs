use mozjpeg::{
    Compress,
    ColorSpace
};
use image::{
    DynamicImage,
    imageops
};
use std::{
    fs::{
        write,
        create_dir_all
    },
    env::args
};
use walkdir::WalkDir;
use rayon::prelude::*;

fn mini_size(input: String, output: String) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(img) = image::open(input) {
        let img = imageops::resize(
            &img,
            177,
            256,
            imageops::FilterType::Lanczos3
        );
        let img = DynamicImage::from(img);
        let rgb_img = img.to_rgb8();
        let mut pixels: Vec<u8> = rgb_img.as_raw().to_vec();
        let (width, height) = rgb_img.dimensions();
        let mut comp = Compress::new(ColorSpace::JCS_RGB);
        comp.set_size((width) as usize, (height) as usize);
        comp.set_quality(80.0);
        let mut vec: Vec<u8> = Vec::new();
        if let Ok(mut comp_started) = comp.start_compress(&mut vec) {
            if let Ok(_) = comp_started.write_scanlines(&mut pixels) {
                if let Ok(writer) = comp_started.finish() {
                    write(output, writer)?;
                }
            }
        }   
    }
    
    Ok(())
}

fn walk(
	dir: String,
	mut callback: impl FnMut(String, String, String) -> (),
) -> Result<(), String> {
    create_dir_all(&dir).map_err(|e| e.to_string())?;
    for entry in WalkDir::new(dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|n| n.to_str()) {
                        if let Some(ext) = path.extension().and_then(|n| n.to_str()) {
                            if let Some(path) = path.as_os_str().to_str() {
                                callback(ext.to_string(), stem.to_string(), path.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
	Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    let input_dir = args[1].trim().to_string();
    let output_dir = args[2].trim().to_string();
    let mut files: Vec<(String, String)> = Vec::new();
    let _ = create_dir_all(&output_dir).map_err(|e| e.to_string());
    let _ = walk(input_dir, |ext, stem, path|{
        let output: String = format!("{}/{}.{}", output_dir, stem, ext);
        files.push((path, output));
    });
    files.par_iter().for_each(|(path, output)| {
        let _ = mini_size(path.clone(), output.clone());
    });
}
