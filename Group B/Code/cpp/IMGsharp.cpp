#include <iostream>
#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgcodecs.hpp>
#include <vector>
#include <filesystem>

namespace fs = std::filesystem;

int main() {
    std::string source_directory = "../../opencv_dataset/Blurry";
    std::string destination_directory = "ImgSharpDone";

    int64 start_time = cv::getTickCount();

    for (const auto& entry : fs::directory_iterator(source_directory)) {
        std::string filename = entry.path().filename().string();
        std::string extension = fs::path(filename).extension().string();
        if (extension == ".jpg" || extension == ".jpeg" || extension == ".png" || extension == ".bmp" || extension == ".gif") {
            cv::Mat inputImage = cv::imread(entry.path().string(), cv::IMREAD_COLOR);

            if (!inputImage.empty()) {
                cv::Mat channels[3];
                cv::split(inputImage, channels);

                for (int i = 0; i < 3; i++) {
                    cv::Mat sharpImage;
                    cv::Laplacian(channels[i], sharpImage, CV_16S, 3);
                    cv::convertScaleAbs(sharpImage, sharpImage);
                    channels[i] = sharpImage;
                }

                cv::Mat sharpenedImage;
                cv::merge(channels, 3, sharpenedImage);

                std::string destination_path = destination_directory + "/" + filename;
                cv::imwrite(destination_path, sharpenedImage);

                std::cout << "Sharpened and saved: " << destination_path << std::endl;
            }
        }
    }

    double end_time = cv::getTickCount();
    double execution_time = (end_time - start_time) / cv::getTickFrequency();

    std::cout << "Image sharpened and saved:" << destination_directory << std::endl;
    std::cout << "Execution Time: " << execution_time << " seconds" << std::endl;

    return 0;
}
