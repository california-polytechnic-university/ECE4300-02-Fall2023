use opencv::{imgcodecs, imgproc, core, types};
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

    let input_folder = "../../../../opencv_dataset/Blurry";
    let output_folder = "../../IMGsharpDone";

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
                    if let Ok(image) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
                        let kernel = core::Mat::from_slice(&[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0])?;

                        let mut sharpened_image = core::Mat::default();
                        imgproc::filter_2d(&image, &mut sharpened_image, core::CV_16S, &kernel, core::Point::new(-1, -1), 0.0, core::BORDER_DEFAULT)?;

                        let mut converted_image = core::Mat::default();
                        core::convert_scale_abs(&sharpened_image, &mut converted_image, 1.0, 0.0)?;

                        let file_name = path.file_name().unwrap().to_string_lossy();
                        let output_path = format!("{}/{}", output_folder, file_name);

                        imgcodecs::imwrite(&output_path, &converted_image, &types::VectorOfi32::new())?;

                        println!("Image sharpened and saved:: {}", output_path);
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
