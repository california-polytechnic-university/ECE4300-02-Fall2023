import cv2
import os

source_directory = "../opencv_dataset/Blurry"
destination_directory = "IMGSharpDone"
os.makedirs(destination_directory, exist_ok=True)

start_time = cv2.getTickCount()

laplacian_kernel = 3

for filename in os.listdir(source_directory):
    if filename.lower().endswith(('.jpg', '.jpeg', '.png', '.bmp', '.gif')):
        input_image = cv2.imread(os.path.join(source_directory, filename))

        if input_image is not None:
            channels = cv2.split(input_image)
            sharpened_channels = []

            for channel in channels:
                sharp_channel = cv2.Laplacian(channel, cv2.CV_16S, ksize=laplacian_kernel)
                sharp_channel = cv2.convertScaleAbs(sharp_channel)
                sharpened_channels.append(sharp_channel)

            sharpened_image = cv2.merge(sharpened_channels)

            destination_path = os.path.join(destination_directory, filename)
            cv2.imwrite(destination_path, sharpened_image)

            print(f"Image harpened and saved: {destination_path}")

end_time = cv2.getTickCount()
execution_time = (end_time - start_time) / cv2.getTickFrequency()

print(f"Image sharpening. Sharpened images saved in '{destination_directory}'.")
print(f"Execution Time: {execution_time} seconds")
