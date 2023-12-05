package main

import (
	"fmt"
	"gocv.io/x/gocv"
	"path/filepath"
	"image"
	_ "image/jpeg"
)

func main() {
	startTime := gocv.GetTickCount()

	inputFolder := "../../opencv_dataset/yolo_objects"
	outputFolder := "resizeDone"

	targetWidth := 1600
	targetHeight := 1200

	files, err := filepath.Glob(filepath.Join(inputFolder, "*.jpg"))
	if err != nil {
		fmt.Printf("Error reading input folder: %v\n", err)
		return
	}

	for _, file := range files {
		img := gocv.IMRead(file, gocv.IMReadColor)
		if img.Empty() {
			fmt.Println("Error reading image:", file)
			return
		}

		resizedImage := gocv.NewMat()
		gocv.Resize(img, &resizedImage, image.Point{X: targetWidth, Y: targetHeight}, 0, 0, gocv.InterpolationArea)

		destinationPath := filepath.Join(outputFolder, filepath.Base(file))
		gocv.IMWrite(destinationPath, resizedImage)

		fmt.Println("Resized and saved: ", destinationPath)

		img.Close()
		resizedImage.Close()
	}

	endTime := gocv.GetTickCount()
	executionTime := (endTime - startTime) / gocv.GetTickFrequency()

	fmt.Printf("Execution Time: %v seconds\n", executionTime)
}
