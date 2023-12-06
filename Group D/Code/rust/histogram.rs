use opencv::{core, imgcodecs, imgproc, prelude::*, types};
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
                   
                    if let Ok(image) =
                        imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)
                    {
                        
                        let mut channels: core::Vector<Mat> = core::Vector::new();
                        core::split(&image, &mut channels).unwrap();

                       
                        let hist_size: core::Vector<i32> = core::Vector::from_iter(vec![256]);
                        let hist_range = types::VectorOff32::from_slice(&[0.0, 256.0]);
                        let accumulate = false;

                        let mut histograms: core::Vector<Mat> = core::Vector::new();
                        for i in 0..channels.len() {
                            if let Ok(channel) = channels.get(i) {
                                let mut hist = Mat::default();
                    
                                imgproc::calc_hist(
                                    &channel,
                                    &core::Vector::from_iter(vec![0]),
                                    &Mat::default(),
                                    &mut hist,
                                    &hist_size,
                                    &hist_range,
                                    accumulate,
                                )
                                .map_err(|error| CustomError::OpenCVError(error))?;
                                histograms.push(hist);
                            } else {
                              
                            }
                        }

                        
                        let mut plot = Mat::new_rows_cols_with_default(
                            800,
                            600,
                            core::CV_8UC3,
                            core::Scalar::new(255.0, 255.0, 255.0, 0.0),
                        )?;
                        plot.set_to_def(&core::Scalar::new(255.0, 255.0, 255.0, 0.0))?;

                        let colors = types::VectorOfScalar::from_iter(vec![
                            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
                            core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                            core::Scalar::new(255.0, 0.0, 0.0, 0.0),
                        ]);

                        for i in 0..histograms.len() {
                            let mut normalized_hist = Mat::default();
                            core::normalize(
                                &histograms.get(i)?,
                                &mut normalized_hist,
                                0.0,
                                plot.rows() as f64 - 50.0,
                                core::NORM_MINMAX,
                                -1,
                                &Mat::default(),
                            )?;

                            for j in 1..normalized_hist.rows() {
                                let pt1 = core::Point::new(
                                    (j - 1) as i32,
                                    plot.rows() as i32
                                        - 50
                                        - *normalized_hist.at_2d::<f32>(j - 1, 0)? as i32,
                                );
                                let pt2 = core::Point::new(
                                    j as i32,
                                    plot.rows() as i32
                                        - 50
                                        - *normalized_hist.at_2d::<f32>(j, 0)? as i32,
                                );

                                imgproc::line(&mut plot, pt1, pt2, colors.get(i)?, 2, 8, 0)?;
                            }
                        }

                       
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        let output_path = format!("{}/{}", output_folder, file_name);
                        imgcodecs::imwrite(&output_path, &plot, &types::VectorOfi32::new())?;

                        println!("Histogram image saved: {}", output_path);
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
