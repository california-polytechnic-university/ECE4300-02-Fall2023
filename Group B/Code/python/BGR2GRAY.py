import cv2
import os

input_folder = "../opencv_dataset/Blue"
output_folder = "B2GDone"
os.makedirs(output_folder, exist_ok=True)

start_time = cv2.getTickCount()

for filename in os.listdir(input_folder):
    if filename.endswith(".jpg") or filename.endswith(".jpeg"):
        image_path = os.path.join(input_folder, filename)
        image = cv2.imread(image_path)

        if image is not None:
            converted_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
            output_path = os.path.join(output_folder, filename)
            cv2.imwrite(output_path, converted_image)
            print(f"Image converted, new image saved: {output_path}")

end_time = cv2.getTickCount()
execution_time = (end_time - start_time) / cv2.getTickFrequency()

print(f"Execution Time: {execution_time} seconds")
