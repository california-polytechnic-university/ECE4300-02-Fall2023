import cv2
import os
import time

input_folder = "../opencv_dataset/yolo_objects"
output_folder = "IMGcropDone"
os.makedirs(output_folder, exist_ok=True)

start_time = cv2.getTickCount()

for filename in os.listdir(input_folder):
    image_path = os.path.join(input_folder, filename)
    image = cv2.imread(image_path)

    width = int(image.shape[1] * 0.25)
    height = int(image.shape[0] * 0.25)

    startX = (image.shape[1] - width) // 2
    startY = (image.shape[0] - height) // 2

    if startX + width > image.shape[1]:
        width = image.shape[1] - startX
    if startY + height > image.shape[0]:
        height = image.shape[0] - startY

    cropped = image[startY:startY+height, startX:startX+width]

    output_path = os.path.join(output_folder, filename)
    cv2.imwrite(output_path, cropped)
    
    print(f"Image cropped, new image saved: {output_path}")

end_time = cv2.getTickCount()
execution_time = (end_time - start_time) / cv2.getTickFrequency()

print(f"Execution Time: {execution_time} seconds")
