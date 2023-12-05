#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <iostream>
#include <filesystem>

namespace fs = std::filesystem;

int main() {
    std::string input_folder = "../../opencv_dataset/Blue";
    std::string output_folder = "B2GDone";
    fs::create_directories(output_folder);

    double start_time = static_cast<double>(cv::getTickCount());

    for (const auto& entry : fs::directory_iterator(input_folder)) {
        std::string filename = entry.path().filename();

        if (filename.size() >= 4 &&
            (filename.compare(filename.size() - 4, 4, ".jpg") == 0 || filename.compare(filename.size() - 5, 5, ".jpeg") == 0)) {

            std::string image_path = entry.path();
            cv::Mat image = cv::imread(image_path);

            if (!image.empty()) {
                cv::Mat converted_image;
                cv::cvtColor(image, converted_image, cv::COLOR_BGR2GRAY); 

                std::string output_path = output_folder + "/" + filename;
                cv::imwrite(output_path, converted_image);
                std::cout << "Image converted, new image saved: " << output_path << std::endl;
            }
        }
    }

    double end_time = static_cast<double>(cv::getTickCount());
    double execution_time = (end_time - start_time) / cv::getTickFrequency();

    std::cout << "Execution Time: " << execution_time << " seconds" << std::endl;

    return 0;
}
