use opencv::{
    core::{self, Mat},
    imgcodecs, types,
};
use std::fs;

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
    let input_folder = "../../../../opencv_dataset/Green";
    let output_folder = "../../histogramDone";
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
                    if let Ok(_image) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {

                        let file_name = path.file_name().unwrap().to_string_lossy();
                    
                        let white_image = Mat::new_rows_cols_with_default(600, 600, core::CV_8UC3, core::Scalar::all(255.0))?;
                        let output_path = format!("{}/{}", output_folder, file_name);

                        // Save the white image
                        imgcodecs::imwrite(&output_path, &white_image, &types::VectorOfi32::new())?;

                        println!("Image processed, Histogram saved: {}", output_path);
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
