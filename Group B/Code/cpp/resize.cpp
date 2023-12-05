#include <iostream>
#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/core/core.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <vector>
#include <string>
#include <filesystem>

namespace fs = std::filesystem;

int main() {
    std::string source_directory = "../../opencv_dataset/yolo_objects";
    std::string destination_directory = "resizeDone";
    int target_width = 1600;
    int target_height = 1200;

    double start_time = cv::getTickCount();

    for (const auto& entry : fs::directory_iterator(source_directory)) {
        std::string filename = entry.path().filename().string();
        std::string extension = fs::path(filename).extension().string();
        if (extension == ".jpg" || extension == ".jpeg" || extension == ".png" || extension == ".bmp" || extension == ".gif") {
            cv::Mat image = cv::imread(entry.path().string());

            if (!image.empty()) {
                cv::Mat resized_image;
                cv::resize(image, resized_image, cv::Size(target_width, target_height));

                std::string destination_path = destination_directory + "/" + filename;
                cv::imwrite(destination_path, resized_image);

                std::cout << "Resized and saved: " << destination_path << std::endl;
            }
        }
    }

    double end_time = cv::getTickCount();
    double execution_time = (end_time - start_time) / cv::getTickFrequency();

    std::cout << "Execution Time: " << execution_time << " seconds" << std::endl;

    return 0;
}
