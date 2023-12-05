use opencv::{imgcodecs, imgproc, core, types};
use std::fs;
use std::env;

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

    let input_folder = "../../../../opencv_dataset/Blue";
    let output_folder = "../../B2GDone";

    let entries = fs::read_dir(input_folder)?;
    let tick_frequency = core::get_tick_frequency()?;
    let start_time = core::get_tick_count()?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext == "jpg" || ext == "jpeg" {
                if let Ok(mut image) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
                    let mut gray_image = core::Mat::default();
                    imgproc::cvt_color(&mut image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0);

                    let file_name = path.file_name().unwrap().to_string_lossy();
                    let output_path = format!("{}/{}", output_folder, file_name);

                    imgcodecs::imwrite(&output_path, &gray_image, &types::VectorOfi32::new())?;
                    println!("Image converted, new image saved: {}", output_path);
                }
            }
        }
    }

    let end_time = core::get_tick_count()?;
    let execution_time = (end_time as f64 - start_time as f64) / tick_frequency;

    println!("Execution Time: {:.6} seconds", execution_time);

    Ok(())
}
