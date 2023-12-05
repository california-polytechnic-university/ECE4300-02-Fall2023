import os
import cv2

source_directory = '../opencv_dataset/yolo_objects'
destination_directory = 'resizeDone'
target_width = 1600
target_height = 1200
files = os.listdir(source_directory)

start_time = cv2.getTickCount()

for filename in files:
    if filename.endswith(('.jpg', '.jpeg', '.png', '.bmp', '.gif')):
        image = cv2.imread(os.path.join(source_directory, filename))
        
        if image is not None:
            resized_image = cv2.resize(image, (target_width, target_height))
            destination_path = os.path.join(destination_directory, filename)
            cv2.imwrite(destination_path, resized_image)
            
            print(f'Resized and saved: {destination_path}')

end_time = cv2.getTickCount()
execution_time = (end_time - start_time) / cv2.getTickFrequency()

print(f"Execution Time: {execution_time} seconds")
