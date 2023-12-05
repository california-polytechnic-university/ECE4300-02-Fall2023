#include <iostream>
#include <filesystem>
#include <opencv2/opencv.hpp>

int main() {
    double startTime = cv::getTickCount();

    std::string inputFolder = "../../opencv_dataset/yolo_objects";
    std::string outputFolder = "IMGcropDone";
    std::filesystem::create_directories(outputFolder);

    for (const auto& entry : std::filesystem::directory_iterator(inputFolder)) {
        std::string file = entry.path().string();
        cv::Mat img = cv::imread(file);

        if (img.empty()) {
            std::cout << "Error reading image" << std::endl;
            return -1;
        }

        int width = static_cast<int>(img.cols * 0.25);
        int height = static_cast<int>(img.rows * 0.25);

        int startX = (img.cols - width) / 2;
        int startY = (img.rows - height) / 2;

        if (startX + width > img.cols) {
            width = img.cols - startX;
        }
        if (startY + height > img.rows) {
            height = img.rows - startY;
        }

        cv::Rect roi(startX, startY, width, height);
        cv::Mat cropped = img(roi);

        std::string outputFile = outputFolder + "/" + entry.path().filename().string();
        if (!cv::imwrite(outputFile, cropped)) {
            std::cout << "Error writing image: " << outputFile << std::endl;
            continue;
        }
        std::cout << "Image cropped, new image saved: " << outputFile << std::endl;
    }

    double endTime = cv::getTickCount();
    double elapsedTime = (endTime - startTime) / cv::getTickFrequency();

    std::cout << "Execution Time: " << elapsedTime << " seconds" << std::endl;

    return 0;
}
