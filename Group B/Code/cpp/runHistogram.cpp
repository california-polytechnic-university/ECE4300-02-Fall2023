#include <iostream>
#include <filesystem>
#include <opencv2/opencv.hpp>

int main() {
    double startTime = cv::getTickCount();

    std::string inputFolder = "../../opencv_dataset/Blurry";
    std::string outputFolder = "histogramDone";
    std::filesystem::create_directories(outputFolder);

    for (const auto& entry : std::filesystem::directory_iterator(inputFolder)) {
        std::string file = entry.path().string();

        cv::Mat img = cv::imread(file, cv::IMREAD_COLOR);

        if (img.empty()) {
            continue;
        }

        std::vector<cv::Mat> channels;
        cv::split(img, channels);

        int histSize[] = {256};
        float range[] = {0, 256};
        const float* histRange[] = {range};
        bool uniform = true;
        bool accumulate = false;
        std::vector<cv::Mat> histograms(channels.size());
        for (size_t i = 0; i < channels.size(); ++i) {
            cv::calcHist(&channels[i], 1, 0, cv::Mat(), histograms[i], 1, histSize, histRange, uniform, accumulate);
        }

        cv::Mat plot(800, 600, CV_8UC3, cv::Scalar(255, 255, 255));
        plot.setTo(cv::Scalar(255, 255, 255));
        std::vector<cv::Scalar> colors = {cv::Scalar(0, 0, 255), cv::Scalar(0, 255, 0), cv::Scalar(255, 0, 0)};
        for (size_t i = 0; i < histograms.size(); ++i) {
            cv::normalize(histograms[i], histograms[i], 0.0, plot.rows - 50, cv::NORM_MINMAX);
            for (int j = 1; j < histograms[i].rows; ++j) {
                cv::line(plot, cv::Point(j - 1, plot.rows - 50 - static_cast<int>(histograms[i].at<float>(j - 1))),
                         cv::Point(j, plot.rows - 50 - static_cast<int>(histograms[i].at<float>(j))), colors[i], 2);
            }
        }

        std::string outputFile = outputFolder + "/" + entry.path().filename().string();
        cv::imwrite(outputFile, plot);
        std::cout << "Image processed, Histogram saved: " << outputFile << std::endl;

    }

    double endTime = cv::getTickCount();
    double elapsedTime = (endTime - startTime) / cv::getTickFrequency();
std::cout << "Execution Time: " << elapsedTime << " seconds" << std::endl;
    return 0;
}
