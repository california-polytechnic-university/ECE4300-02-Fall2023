from ultralytics import YOLO
model = YOLO("yolov8m.pt")

model.train(data = "data.yaml", batch = 64, imgsz=640, epochs=100, workers=8)