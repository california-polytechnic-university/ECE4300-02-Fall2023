use opencv::{
    core::{self, Mat, Rect},
    imgcodecs, types,
    prelude::*,
};
use std::{env, fs};

#[derive(Debug)]
enum CustomError {
    OpenCVError(opencv::Error),
    IOError(std::io::Error),
}

impl From<opencv::Error> for CustomError {
    fn from(error: opencv::Error) -> Self {
        CustomError::OpenCVError(error)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IOError(error)
    }
}

fn main() -> Result<(), CustomError> {
    if let Ok(cwd) = env::current_dir() {
        println!("Current working directory: {:?}", cwd);
    } else {
        eprintln!("Failed to get the current working directory.");
    }

    let input_folder = "../../../../opencv_dataset/yolo_objects";
    let output_folder = "../../IMGcropDone";

    fs::create_dir_all(output_folder)?;

    let entries = fs::read_dir(input_folder)?;
    let tick_frequency = core::get_tick_frequency()?;
    let start_time = core::get_tick_count()?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "jpg" || ext == "jpeg" {
                    if let Ok(mut img) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
                        let width = (img.cols() as f64 * 0.25) as i32;
                        let height = (img.rows() as f64 * 0.25) as i32;

                        let start_x = (img.cols() - width) / 2;
                        let start_y = (img.rows() - height) / 2;

                        let width = if start_x + width > img.cols() {
                            img.cols() - start_x
                        } else {
                            width
                        };

                        let height = if start_y + height > img.rows() {
                            img.rows() - start_y
                        } else {
                            height
                        };

                        let roi = Rect::new(start_x, start_y, width, height);
                        let cropped = Mat::roi(&mut img, roi)?;

                        let file_name = path.file_name().unwrap().to_string_lossy();
                        let output_path = format!("{}/{}", output_folder, file_name);

                        imgcodecs::imwrite(&output_path, &cropped, &types::VectorOfi32::new())?;
                        println!("Image cropped, new image saved: {}", output_path);
                    }
                }
            }
        }
    }

    let end_time = core::get_tick_count()?;
    let execution_time = ((end_time as f64) - (start_time as f64)) / tick_frequency;

    println!("Execution Time: {:.6} seconds", execution_time);

    Ok(())
}
