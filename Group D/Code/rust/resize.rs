use opencv::{imgcodecs, imgproc, core, types};
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
    let input_folder = "../../../../opencv_dataset/yolo_objects";
    let output_folder = "../../resizeDone";

    fs::create_dir_all(output_folder)?;

    let entries = fs::read_dir(input_folder)?;
    let tick_frequency = core::get_tick_frequency()?;
    let start_time = core::get_tick_count()?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "bmp" || ext == "gif" {
                    if let Ok(image) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
                        let mut resized_image = image.clone();

                        imgproc::resize(
                            &image,
                            &mut resized_image,
                            core::Size {
                                width: 1600,
                                height: 1200,
                            },
                            0.0,
                            0.0,
                            imgproc::INTER_LINEAR,
                        )?;

                        let file_name = path.file_name().unwrap().to_string_lossy();
                        let output_path = format!("{}/{}", output_folder, file_name);

                        imgcodecs::imwrite(&output_path, &resized_image, &types::VectorOfi32::new())?;

                        println!("Resized and saved: {}", output_path);
                    }
                }
            }
        }
    }

    let end_time = core::get_tick_count()?;
    let execution_time = (end_time as f64 - start_time as f64) / tick_frequency;

    println!("Execution Time: {:.6} seconds", execution_time);

    Ok(())
}
