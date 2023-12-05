import cv2
import os
import numpy as np

input_folder = "../opencv_dataset/Blurry"
output_folder = "histogramDone"
os.makedirs(output_folder, exist_ok=True)

start_time = cv2.getTickCount()

for filename in os.listdir(input_folder):
    if filename.endswith(".jpg") or filename.endswith(".jpeg"):
        image_path = os.path.join(input_folder, filename)
        image = cv2.imread(image_path)

        if image is not None:
            channels = cv2.split(image)

        histSize = [256]
        histRange = [0, 256]
        uniform = True
        accumulate = False
        histograms = []
        for i in range(len(channels)):
            hist = cv2.calcHist([channels[i]], [0], None, histSize, histRange, accumulate=accumulate)
            histograms.append(hist)

        plot = np.ones((800, 600, 3)) * 255
        colors = [(0, 0, 255), (0, 255, 0), (255, 0, 0)]
        for i in range(len(histograms)):
            cv2.normalize(histograms[i], histograms[i], 0, plot.shape[0] - 50, cv2.NORM_MINMAX)
            for j in range(1, len(histograms[i])):
                cv2.line(plot, (j - 1, plot.shape[0] - 50 - int(histograms[i][j - 1])),
                         (j, plot.shape[0] - 50 - int(histograms[i][j])), colors[i], 2)
            
        output_path = os.path.join(output_folder, filename)
        cv2.imwrite(output_path, plot)

        print(f"Image processed, Histogram saved: {output_path}")

end_time = cv2.getTickCount()
execution_time = (end_time - start_time) / cv2.getTickFrequency()

print(f"Execution Time: {execution_time} seconds")
