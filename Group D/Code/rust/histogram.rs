use opencv::{core, imgcodecs, imgproc, types, prelude::*};
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
    let input_folder = "../../../../opencv_dataset/Blurry";
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
                    if let Ok(image) = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
                        let mut channels: core::Vector<Mat> = core::Vector::new();
                        core::split(&image, &mut channels).unwrap();

                        let hist_size: core::Vector<i32> = core::Vector::from_iter(vec![256]);
                        let hist_range = types::VectorOff32::from_slice(&[0.0, 256.0]);
                        let accumulate = false;

                        for i in 0..channels.len() {
                            if let Ok(channel) = channels.get(i) {
                                let mut hist = Mat::default();

                                if channel.empty() {
                                    println!("Channel is empty or contains no data.");
                                } else {
                                    match imgproc::calc_hist(
                                        &channel,
                                        &core::Vector::from_iter(vec![0]),
                                        &Mat::default(),
                                        &mut hist,
                                        &hist_size,
                                        &hist_range,
                                        accumulate,
                                    ) {
                                        Ok(_) => {
                                            println!("Histogram calculated for channel {}", i);

                                            
                                            let hist_image = Mat::new_rows_cols(100, 256, opencv::core::CV_8UC1, core::Scalar::all(255.0))?;

                                          
                                            let max_value = hist.min_max_loc()?.max_val;
                                            let scale = 0.9 * 100 as f64 / max_value as f64;

                                            for j in 0..255 {
                                                let value = hist.at_2d::<f64>(j, 0)?;
                                                let next_value = hist.at_2d::<f64>(j + 1, 0)?;
                                                imgproc::line(
                                                    &hist_image,
                                                    core::Point::new(j, 100 - (value * scale) as i32),
                                                    core::Point::new(j + 1, 100 - (next_value * scale) as i32),
                                                    core::Scalar::all(0.0),
                                                    1,
                                                    imgproc::LINE_8,
                                                    0,
                                                )?;
                                            }

                                            let file_name = path.file_name().unwrap().to_string_lossy();
                                            let output_path = format!("Image processed, Histogram saved:", output_folder, file_name);
                                            imgcodecs::imwrite(&output_path, &hist_image, &core::Vector::<i32>::new())?;
                                            println!("Image processed, Histogram saved: {}", output_path);
                                        }
                                        Err(e) => {
                                            println!("Error calculating histogram for channel {}: {:?}", i, e);
                                            return Err(CustomError::OpenCVError(e));
                                        }
                                    }
                                }
                            } else {
                                println!("Error getting channel {}", i);
                            }
                        }
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
